//  BSTOWIR.rs
//    by Lut99
//
//  Created:
//    08 Feb 2024, 10:41:34
//  Last edited:
//    08 Feb 2024, 14:07:42
//  Auto updated?
//    Yes
//
//  Description:
//!   Implements the converter from BraneScript (BS) to the Workflow
//!   Intermediate Representation (WIR).
//

use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};
use std::path::PathBuf;
use std::sync::Arc;

use axum::http::StatusCode;
use brane_ast::ast::{ComputeTaskDef, Edge, TaskDef};
use brane_ast::func_id::FunctionId;
use brane_ast::locations::Locations;
use brane_ast::{CompileResult, ParserOptions, SymTable, Workflow};
use brane_exe::pc::ProgramCounter;
use brane_shr::formatters::BlockFormatter;
use enum_debug::EnumDebug;
use error_trace::trace;
use log::debug;
use serde::{Deserialize, Serialize};
use specifications::address::Address;
use specifications::data::DataIndex;
use specifications::package::PackageIndex;


/***** ERRORS *****/
/// Defines errors relating to fetching [`Package`](PackageIndex)- and [`DataIndex`]es.
#[derive(Debug)]
enum IndexError {
    /// Failed to fetch the index from a local test directory ([`IndexSource::LocalTest`]).
    ///
    /// The `kind`-field indicates if this is a package- or data-index.
    LocalTest { kind: &'static str, path: PathBuf },
    /// Failed to fetch the index from a local client directory ([`IndexSource::LocalClient`]).
    ///
    /// The `kind`-field indicates if this is a package- or data-index.
    LocalClient { kind: &'static str, path: PathBuf, err: brane_tsk::local::Error },
    /// Failed to fetch the index from a remote registry ([`IndexSource::Remote`]).
    ///
    /// The `kind`-field indicates if this is a package- or data-index.
    Remote { kind: &'static str, url: String, err: brane_tsk::api::Error },
}
impl Display for IndexError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use IndexError::*;
        match self {
            LocalTest { kind, path } => write!(f, "Failed to load {} index from local test directory '{}' (see panic above)", kind, path.display()),
            LocalClient { kind, path, .. } => write!(f, "Failed to load {} index from local client directory '{}'", kind, path.display()),
            Remote { kind, url, .. } => write!(f, "Failed to load {kind} index from remote registry at '{url}'"),
        }
    }
}
impl Error for IndexError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use IndexError::*;
        match self {
            LocalTest { .. } => None,
            LocalClient { err, .. } => Some(err),
            Remote { err, .. } => Some(err),
        }
    }
}

/// Defines errors originating in [`analyze_wir_and_trivially_plan()`].
#[derive(Debug)]
enum AnalyzeError {
    /// A node was found with an ambigious location.
    AmbigiousLocation { id: String, node_func_id: String, node_edge_idx: usize, def: ComputeTaskDef },
    /// We encountered a non compute-task in the workflow.
    IllegalTransferTask { id: String, node: ProgramCounter },
    /// A given edge index was out-of-range for the given function.
    UnknownEdgeIdx { id: String, func_id: FunctionId, edge_idx: usize, max: usize },
    /// A given task ID was not known for the given workflow.
    UnknownTaskId { id: String, node: ProgramCounter, task_id: usize },
}
impl Display for AnalyzeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use AnalyzeError::*;
        match self {
            AmbigiousLocation { id, node_func_id, node_edge_idx, def } => {
                write!(
                    f,
                    "Encountered call to task '{}<{}>::{}' at location {}:{} in workflow '{id}' that has ambigious location (please scope it to a \
                     single location by adding '#[on(\"<LOCATION>\")]' above the call",
                    def.package, def.version, def.function.name, node_func_id, node_edge_idx
                )
            },
            IllegalTransferTask { id, node } => write!(f, "Task call at '{node}' in workflow '{id}' is a deprecated Transfer-task"),
            UnknownEdgeIdx { id, func_id, edge_idx, max } => {
                write!(f, "Edge index {edge_idx} is out-of-bounds for function {func_id} in workflow '{id}' that has {max} edges")
            },
            UnknownTaskId { id, node, task_id } => write!(f, "Unknown task ID '{task_id}' referenced in node '{node}' in workflow '{id}'"),
        }
    }
}
impl Error for AnalyzeError {}





/***** HELPER FUNCTIONS *****/
/// Fetches a [`PackageIndex`] from the location pointed to by the given [`IndexSource`].
///
/// # Arguments
/// - `source`: The [`IndexSource`] that determines where we will get the index from.
///
/// # Returns
/// A [`PackageIndex`] loaded from the given `source`.
///
/// # Errors
/// This function may error if we failed to load from the given source for some reason.
async fn get_package_index(source: IndexSource) -> Result<PackageIndex, IndexError> {
    // Match on the kind to call the correct function
    match source {
        IndexSource::LocalTest(path) => {
            debug!("Fetching package index from local test directory '{}'", path.display());
            std::panic::catch_unwind(|| brane_shr::utilities::create_package_index_from(&path))
                .map_err(|_| IndexError::LocalTest { kind: "package", path })
        },
        IndexSource::LocalClient(path) => {
            debug!("Fetching package index from local client directory '{}'", path.display());
            brane_tsk::local::get_package_index(&path).map_err(|err| IndexError::LocalClient { kind: "package", path, err })
        },
        IndexSource::Remote(addr) => {
            let url: String = format!("{addr}/graphql");
            debug!("Fetching package index from remote registry at '{url}'");
            brane_tsk::api::get_package_index(&url).await.map_err(|err| IndexError::Remote { kind: "package", url, err })
        },
    }
}

/// Fetches a [`DataIndex`] from the location pointed to by the given [`IndexSource`].
///
/// # Arguments
/// - `source`: The [`IndexSource`] that determines where we will get the index from.
///
/// # Returns
/// A [`DataIndex`] loaded from the given `source`.
///
/// # Errors
/// This function may error if we failed to load from the given source for some reason.
async fn get_data_index(source: IndexSource) -> Result<DataIndex, IndexError> {
    // Match on the kind to call the correct function
    match source {
        IndexSource::LocalTest(path) => {
            debug!("Fetching data index from local test directory '{}'", path.display());
            std::panic::catch_unwind(|| brane_shr::utilities::create_data_index_from(&path)).map_err(|_| IndexError::LocalTest { kind: "data", path })
        },
        IndexSource::LocalClient(path) => {
            debug!("Fetching data index from local client directory '{}'", path.display());
            brane_tsk::local::get_data_index(&path).map_err(|err| IndexError::LocalClient { kind: "data", path, err })
        },
        IndexSource::Remote(addr) => {
            let url: String = format!("{addr}/data/info");
            debug!("Fetching data index from remote registry at '{url}'");
            brane_tsk::api::get_data_index(&url).await.map_err(|err| IndexError::Remote { kind: "data", url, err })
        },
    }
}



/// Given a workflow, traverses it to:
/// - Trivially plan it (i.e., if the workflow itself dictates only one location, assign it; else, throw an error);
/// - Collect all tasks occurring in it (including all inputs to those tasks); and
/// - Find the final result to the workflow if any.
///
/// # Arguments
/// - `wf_id`: A reference to the ID of the workflow (used for debugging purposes only).
/// - `func_id`: The identifier of the function we're currently analyzing (used for debugging purposes only).
/// - `edges`: A mutable reference to the list of edges to plan/analyze.
/// - `pc`: The program counter that points to the current edge we're analyzing in `edges`.
/// - `breakpoint`: If [`Some(...)`], then the analysis will stop once `pc` == `breakpoint`.
/// - `is_main`: If `true`, then we're operating in the toplevel script where return statements exit the workflow.
/// - `tasks`: A list of tasks that will be populated with the ones found in this workflow.
/// - `result`: A list of possible results returned from a workflow.
///
/// # Errors
/// This function may fail if we failed to analyze the workflow.
fn analyze_wir_and_trivially_plan(
    wf_id: &str,
    table: &SymTable,
    func_id: FunctionId,
    edges: &mut [Edge],
    mut pc: usize,
    breakpoint: Option<usize>,
    is_main: bool,
    tasks: &mut Vec<TaskCallInfo>,
    results: &mut Vec<String>,
) -> Result<(), AnalyzeError> {
    loop {
        // Stop on breakpoints
        if let Some(breakpoint) = breakpoint {
            if pc == breakpoint {
                return Ok(());
            }
        }

        // Find the edge
        let edge: &mut Edge = match edges.get_mut(pc) {
            Some(edge) => edge,
            None => {
                return Err(AnalyzeError::UnknownEdgeIdx { id: wf_id.into(), func_id, edge_idx: pc, max: edges.len() });
            },
        };

        // Match on it!
        log::trace!("Analyzing & trivially planning edge {} (Edge::{})", pc, edge.variant());
        match edge {
            Edge::Node { task, locs, at, input, result: _, metadata: _, next } => {
                let pg: ProgramCounter = ProgramCounter::new(func_id, pc);

                // Resolve the task ID in the workflow table
                let def: &ComputeTaskDef = match table.tasks.get(*task) {
                    Some(def) => match def {
                        TaskDef::Compute(def) => def,
                        TaskDef::Transfer => {
                            return Err(AnalyzeError::IllegalTransferTask { id: wf_id.into(), node: pg });
                        },
                    },
                    None => {
                        return Err(AnalyzeError::UnknownTaskId { id: wf_id.into(), node: pg, task_id: *task });
                    },
                };

                // Plan the task
                debug!("Attempting to plan node '{}' in workflow '{}' (possible locations: {:?})", pg, wf_id, locs);
                match locs {
                    // If there's exactly one location, plan there
                    Locations::Restricted(locs) if locs.len() == 1 => *at = Some(locs.first().unwrap().clone()),

                    _ => {
                        return Err(AnalyzeError::AmbigiousLocation {
                            id: wf_id.into(),
                            node_func_id: if func_id.is_main() {
                                "<main>".into()
                            } else {
                                if let Some(def) = table.funcs.get(func_id.id()) { def.name.clone() } else { format!("{}", func_id.id()) }
                            },
                            node_edge_idx: pc,
                            def: def.clone(),
                        });
                    },
                }

                // Now built the index of it
                tasks.push(TaskCallInfo {
                    pg,
                    name: format!("{}<{}>::{}", def.package, def.version, def.function.name),
                    datasets: input.keys().map(|dataname| dataname.name().into()).collect(),
                });

                // Then we can move to the next edge
                pc = *next;
                continue;
            },
            Edge::Linear { instrs: _, next } => {
                // No need to dive into the instructions (basically expressions)
                pc = *next;
                continue;
            },
            Edge::Stop {} => return Ok(()),

            Edge::Branch { true_next, false_next, merge } => {
                let true_next: usize = *true_next;
                let false_next: Option<usize> = *false_next;
                let merge: Option<usize> = *merge;

                // Recurse into the true-branch
                analyze_wir_and_trivially_plan(wf_id, table, func_id, edges, true_next, merge, is_main, tasks, results)?;
                // Recurse into the false-branch (if any)
                if let Some(false_next) = false_next {
                    analyze_wir_and_trivially_plan(wf_id, table, func_id, edges, false_next, merge, is_main, tasks, results)?;
                }

                // If either branch does not fully return, continue
                if let Some(merge) = merge {
                    pc = merge;
                    continue;
                } else {
                    return Ok(());
                }
            },
            Edge::Parallel { branches, merge } => {
                let branches: Vec<usize> = branches.clone();
                let merge: usize = *merge;

                // Recurse into all the branches
                for b in branches {
                    // NOTE: Switch `is_main` to false, as returns break the branch and not the script
                    analyze_wir_and_trivially_plan(wf_id, table, func_id, edges, b, Some(merge), false, tasks, results)?;
                }

                // Continue with the rest of the program
                pc = merge;
                continue;
            },
            Edge::Join { merge: _, next } => {
                // Nothing to do here but to continue
                pc = *next;
                continue;
            },
            Edge::Loop { cond, body, next } => {
                let cond: usize = *cond;
                let body: usize = *body;
                let next: Option<usize> = *next;

                // This one's gnarly, due to the layout of the edges; pay attention to the breakpoints

                // Recurse into the condition
                analyze_wir_and_trivially_plan(wf_id, table, func_id, edges, cond, Some(body - 1), is_main, tasks, results)?;
                // Recurse into the body
                analyze_wir_and_trivially_plan(wf_id, table, func_id, edges, body, Some(cond), is_main, tasks, results)?;

                // Continue with next
                if let Some(next) = next {
                    pc = next;
                    continue;
                } else {
                    return Ok(());
                }
            },

            Edge::Call { input: _, result: _, next } => {
                // We ignore calls, as we do function body analysis separately
                pc = *next;
                continue;
            },
            Edge::Return { result } => {
                // This is where `is_main` comes into play; if we see a return at the toplevel, it returns from the workflow
                if is_main {
                    results.extend(result.iter().map(|dataname| dataname.name().into()));
                }

                // Then quit
                return Ok(());
            },
        }
    }
}





/***** AUXILLARY *****/
/// Alternates between possible sources of a [`Package`](PackageIndex)- or [`DataIndex`].
#[derive(Clone, Debug, EnumDebug)]
pub enum IndexSource {
    /// The index is sourced from a local test repository of packages- or datasets (as found in `tests/` in this repository, for example).
    LocalTest(PathBuf),
    /// The index is sourced from a local client repository of packages- or datasets, as managed by the `brane` CLI tool.
    LocalClient(PathBuf),
    /// The index is sourced from a `brane-api` registry at the given address.
    Remote(Address),
}



/// The struct returned on the happy path of [`to_wir()`].
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AnalyzedWir {
    /// The [`Workflow`] that we compiled to.
    ///
    /// Note that this workflow is already "trivially" planned.
    pub workflow: Workflow,
    /// A list of task calls that occur in this workflow.
    pub tasks:    Vec<TaskCallInfo>,
    /// If there is any, this defines the result to the workflow as a whole.
    pub results:  Vec<String>,
}

/// A struct describing the info we extract from each task call.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaskCallInfo {
    /// The location of the cal, encoded as a [`ProgramCounter`].
    pub pg: ProgramCounter,
    /// The name of the task called.
    pub name: String,
    /// A list of datasets that are input to the TaskCall.
    pub datasets: Vec<String>,
}





/***** LIBRARY *****/
/// Implements the convertion from BraneScript to the Brane WIR.
///
/// Note that this does not just return a workflow, but also some analysis about the workflow.
///
/// # Arguments
/// - `packages`: The source from which to fetch the package index, encoded as an [`IndexSource`].
/// - `data`: The source from which to fetch the dataset index, encoded as an [`IndexSource`].
/// - `body`: The raw BraneScript snippet to compile.
///
/// # Returns
/// A(n already serialized) [`AnalyzedWir`] that encodes not just the compiled workflow, but also a list of all tasks occurring in it (together with their possible input data) and which data is the result of the workflow (if any).
///
/// # Errors
/// This function may error if we failed to read from the `packages`- or `data`-source, or if the input workflow was malformed.
pub async fn bs_to_wir(packages: IndexSource, data: IndexSource, body: String) -> Result<String, (StatusCode, String)> {
    // Fetch the indices
    let pindex: PackageIndex = match get_package_index(packages).await {
        Ok(index) => index,
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", trace!(("Failed to collect package index"), err)))),
    };
    let dindex: DataIndex = match get_data_index(data).await {
        Ok(index) => index,
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", trace!(("Failed to collect package index"), err)))),
    };

    // Compile the program next
    debug!("Compiling input snippet...\n\nSnippet:\n{}\n", BlockFormatter::new(&body));
    let mut workflow: Workflow = match brane_ast::compile_program(body.as_bytes(), &pindex, &dindex, &ParserOptions::bscript()) {
        // NOTE: We ignore the warnings
        CompileResult::Workflow(wf, _warns) => wf,
        CompileResult::Err(errs) => {
            // Collect the errors in a nice string
            let mut msg: Vec<u8> = Vec::new();
            for err in errs {
                err.prettywrite(&mut msg, "<input>", &body).unwrap_or_else(|_| {
                    panic!("BraneScript compiler error prettyprinter failed to write to in-memory buffer; this should never happen!")
                });
            }

            // Add some generic error
            msg.extend(b"Failed to compile input workflow (see errors above)\n");

            // Return the failure body
            return Err((
                StatusCode::BAD_REQUEST,
                String::from_utf8(msg)
                    .unwrap_or_else(|_| panic!("BraneScript compiler error prettyprinter did not produce valid UTF-8; this should never happen!")),
            ));
        },
        CompileResult::Eof(err) => {
            // Serialize the error
            let mut msg: Vec<u8> = Vec::new();
            err.prettywrite(&mut msg, "<input>", &body).unwrap_or_else(|_| {
                panic!("BraneScript compiler error prettyprinter failed to write to in-memory buffer; this should never happen!")
            });

            // Add some generic error
            msg.extend(b"Failed to compile input workflow (see errors above)\n");

            // Return the failure body
            return Err((
                StatusCode::BAD_REQUEST,
                String::from_utf8(msg)
                    .unwrap_or_else(|_| panic!("BraneScript compiler error prettyprinter did not produce valid UTF-8; this should never happen!")),
            ));
        },

        CompileResult::Unresolved(_, _) => unreachable!(),
        CompileResult::Program(_, _) => unreachable!(),
    };

    // Run an analysis to find all nodes and data and the like
    let (mut tasks, mut results): (Vec<TaskCallInfo>, Vec<String>) = (vec![], vec![]);
    {
        /* Step 1: Extract the lists from behind the Arc's to get them mutable */
        // Extract the main graph
        let mut graph: Arc<Vec<Edge>> = Arc::new(vec![]);
        std::mem::swap(&mut workflow.graph, &mut graph);
        let mut graph: Vec<Edge> = Arc::into_inner(graph).unwrap();

        // Extract the function bodies
        let mut funcs: Arc<HashMap<usize, Vec<Edge>>> = Arc::new(HashMap::new());
        std::mem::swap(&mut workflow.funcs, &mut funcs);
        let mut funcs: HashMap<usize, Vec<Edge>> = Arc::into_inner(funcs).unwrap();



        /* Step 2: Run the analysis */
        // Run the main function's analysis
        if let Err(err) =
            analyze_wir_and_trivially_plan(&workflow.id, &workflow.table, FunctionId::Main, &mut graph, 0, None, true, &mut tasks, &mut results)
        {
            return Err((StatusCode::BAD_REQUEST, format!("{}", trace!(("Invalid workflow given"), err))));
        };

        // Analyze/plan all the function bodies
        for (id, body) in funcs.iter_mut() {
            if let Err(err) =
                analyze_wir_and_trivially_plan(&workflow.id, &workflow.table, FunctionId::Func(*id), body, 0, None, false, &mut tasks, &mut results)
            {
                return Err((StatusCode::BAD_REQUEST, format!("{}", trace!(("Invalid workflow given"), err))));
            };
        }



        /* Step 3: Put the lists back */
        // Re-inject the function bodies
        let mut funcs: Arc<HashMap<usize, Vec<Edge>>> = Arc::new(funcs);
        std::mem::swap(&mut funcs, &mut workflow.funcs);

        // Re-inject the main body
        let mut graph: Arc<Vec<Edge>> = Arc::new(graph);
        std::mem::swap(&mut graph, &mut workflow.graph);
    }

    // Now serialize the resulting body
    let res: String = match serde_json::to_string_pretty(&AnalyzedWir { workflow, tasks, results }) {
        Ok(res) => res,
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", trace!(("Failed to serialize workflow"), err)))),
    };

    // Alrighty, return everything
    Ok(res)
}
