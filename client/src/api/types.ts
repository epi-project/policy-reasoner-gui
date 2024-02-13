import { API } from "../context/auth";

export interface PolicyVersion {
    version: number;
}

export interface Policy {
    version: number;
    reasoner: string;
    reasonerVersion: string;
    content: string;
    description: string;
    versionDescription: string;
    jsonContent: string;
}

export interface PolicyContentPostModel {
    reasoner: string;
    reasoner_version: string;
    content: any;
}

export interface PostPolicy {
    description: string;
    version_description: string;
    content: PolicyContentPostModel[],
}

export interface reasonerConnectorInfo {
    hash: string;
    context: {
        version: string,
        type: string
    } & Record<string, any>
}

export interface AuthDataViewmodel{
    policy: string,
    deliberation: string
}

export interface AuthDataPostModel{
    t: API,
    token: string,
}

export enum DeliberationType {
    'task' = 'task',
    'data' = 'data',
    'workflow' = 'workflow',
}

export interface Edge {
    "kind": string,
    "t": number,
}

export interface Task {
    "p": string,
    "v": string,
    "d": {
      "n": string,
    },
}

export interface Workflow {
    graph: Edge[],
    funcs: Record<number, Edge[]>
    table: {
        tasks: Task[]
    }
}

export interface TaskOption {
    "pg": ProgramCounter,
    "name": string,
    "datasets": string[]
}

export interface WorkflowConvResult {
    "workflow": Workflow,
    "tasks": TaskOption[],
    "results": string[]
}

export type ProgramCounter = ["<main>" | number, number]

export interface Option {
    name: string,
    pg: ProgramCounter
}

export enum SYNTAX {
    EFLINT="EFLINT",
    JSON="JSON",
}