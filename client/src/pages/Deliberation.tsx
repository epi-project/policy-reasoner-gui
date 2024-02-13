import React, { FC, useEffect, useRef, useState } from "react"
import { Button, Dialog, DialogActions, DialogContent, DialogContentText, DialogTitle, FormControl, FormLabel, InputLabel, MenuItem, Select, Stack, TextField } from "@mui/material"
import { javascript } from "@codemirror/lang-javascript";
import CodeMirror, { EditorView } from '@uiw/react-codemirror';
import Login from "../components/Login";
import { API } from "../context/auth";
import { convBS, deliberate } from "../api";
import { DeliberationType, Workflow, Option, TaskOption, WorkflowConvResult } from "../api/types";
import { useMutation } from "@tanstack/react-query";
import { AxiosError } from "axios";

interface MenuItemData { 
    id: DeliberationType;
    label: string;
    validate: (task: Option | undefined, dataset: string) => boolean,
    enabled: string[],
    generateRequest: (workflow: Workflow, task?: Option, data?: string) => Promise<any>
}

const menuItems : MenuItemData[] = [{
    id: DeliberationType.task,
    label: 'Execute task request',
    validate: (task: Option | undefined, dataset: string) => {
        return !dataset && !!task
    },
    enabled: ['task'],
    generateRequest: async (workflow: Workflow, task?: Option, data?: string) => {
        return {
            workflow: workflow,
            task_id: task!.pg
        }
    }
}, {
    id: DeliberationType.data,
    label: 'Transfer data request',
    validate: (task: Option | undefined, dataset: string) => {
        return !!(task || dataset)
    },
    enabled: ['task', 'dataset'],
    generateRequest: async (workflow: Workflow, task?: Option, data?: string) => {
        let req: Record<string, any> = {
            workflow: workflow, 
        }

        if (task) {
            req.task_id = task.pg
        }

        if (data) {
            req.data_id = JSON.parse(data!)
        }

        return req
    }
}, {
    id: DeliberationType.workflow,
    label: 'Validate workflow',
    validate: (task: Option | undefined, dataset: string) => {
        return true
    },
    enabled: [],
    generateRequest: async (workflow: Workflow, task?: Option, data?: string) => {
        return {
            workflow: workflow, 
        }
    }
}]

const validate = (item : MenuItemData, task : Option | undefined, dataset : string, workflow: string) : boolean => {
    return !!workflow && item.validate(task, dataset)
}

const handleErr = (error: any, setError: (string) => void) => {
    if (!error) {
        return
    }

    const response = (error as AxiosError).response

    if (!response) {
        let msg = (error as AxiosError).message
        setError(msg || `${msg}`)
        return
    }

    let err = (response.data as any).detail || response.data

    if (!err) {
        setError(`Call returned invalid statuscode: ${response.status} (${response.statusText})`)
        return
    }

    err = `Call returned invalid statuscode: ${response.status} (${response.statusText})

${err}            
` 

    setError(err)
}

const DeliberationPage: FC = () => {
    const [curItem, setCurItem] = useState<MenuItemData>(menuItems[0])
    const [loading, setLoading] = useState<boolean>(false)
    const [task, setTask] = useState<number>(-1)
    const [dataset, setDataset] = useState('')
    const [workflow, setWorkflow] = useState('')
    const [jsonWorkflow, setJsonWorkflow] = useState<Workflow | null>(null)
    const [taskOptions, setTaskOptions] = useState<TaskOption[]>([])
    const [workflowResults, setWorkflowResults] = useState<string[]>([])
    const [response, setResponse] = useState('')
    const [errors, setErrors] = useState<string[]>([])
    const [shownError, setShownError] = useState<string>()
    const [workflowWidth, setWorkflowWidth] = useState(60)
    const [responseWidth, setResponseWidth] = useState(40)
    const contentRef = useRef(null)
    
    useEffect(() => {
        if (!errors.length || !!shownError) {
            return
        }

        setShownError(errors[0])
    }, [errors])

    useEffect(() => {
        const wf = localStorage.getItem('workflow')

        if (wf) {
            updateWorkflow(wf, true)
        }
    }, [])

    const reset = () => {
        setJsonWorkflow(null)
        setTaskOptions([])
        setTask(-1)
        setDataset('')
        setWorkflowResults([])
    }

    const dismissError = () => {
        setShownError(undefined)
        setTimeout(() => setErrors(x => x.slice(1)), 500)
    }
    
    const updateWorkflow = async (wf: string, reportErr: boolean) => {
        console.log('update', reportErr, !wf)
        localStorage.setItem('workflow', wf)
        reset()
        if (!wf) {
            setWorkflow(wf)
            return
        }
        
        try {
            setWorkflow(wf)
            const convResult = await convBS(wf)
            setJsonWorkflow(convResult.workflow)
            setTaskOptions(convResult.tasks)
            setWorkflowResults(convResult.results || [])
        } catch(err) {
            console.log('go and check', reportErr)    
            reportErr && handleErr(err, (err) => {
                console.log('err', err)
                setErrors([...errors, err])
            })
        }
    }

    const updateTask = (tid: number) => {
        setDataset('')
        setTask(tid)
    }

    const canExecute = validate(curItem, task != -1 ? taskOptions[task] : undefined, dataset, workflow)
    const mutation = useMutation({
        mutationFn: deliberate,
        onSuccess: (r) => {
            setResponse(typeof r.data === "string" ? r.data : JSON.stringify(r.data, null, '    '))
        },
        onError: (error) => handleErr(error, setResponse)
    })

    const exec = async  () => {
        const req = await curItem.generateRequest(jsonWorkflow!, task != -1 ? taskOptions[task] : undefined, dataset)
        mutation.mutate({type: curItem.id, req})
    }

    const handleMouseDown = React.useCallback((e: React.MouseEvent) => {
        if (!contentRef.current) {
            return;
        }

        const parent = (contentRef.current as HTMLDivElement).parentElement!;
        const {width, x} = parent.getBoundingClientRect()

        const handleMouseMove = (e) => {
            const correction = 10 / width
            const perc = Math.min(Math.max(0.1, (e.clientX - x) / width), 0.9) - correction
            setWorkflowWidth(x => perc * 100)
            setResponseWidth(x => 100 - perc * 100)
        };

        const handleMouseUp = () => {
            document.removeEventListener('mousemove', handleMouseMove);
            document.removeEventListener('mouseup', handleMouseUp);
        };

        document.addEventListener('mousemove', handleMouseMove);
        document.addEventListener('mouseup', handleMouseUp);
    }, [contentRef.current]);

    return (
        <>
        <div>
            <h1>Deliberation API</h1>
            <div style={{display: 'flex', justifyContent: 'space-between'}}>
                
                <div style={{width: '225px', paddingTop: 50, paddingRight: '25px'}}>
                    <Stack spacing={1}>
                        <FormControl variant="outlined" sx={{ m: 1, minWidth: 120 }}>
                            <InputLabel shrink>Request</InputLabel>
                            <Select
                                sx={{width: '100%'}}
                                value={curItem.id}
                                label="Request"
                                onChange={e => {
                                    updateTask(-1)
                                    setCurItem(menuItems.filter(item => item.id === e.target.value)[0])
                                }}
                            >
                                {menuItems.map(item => <MenuItem key={item.id} value={item.id}>{item.label}</MenuItem>)}
                            </Select>
                        </FormControl>
                        {
                            curItem.enabled.includes('task') ? (
                                <FormControl variant="outlined" sx={{ m: 1, minWidth: 120 }}>
                                    <InputLabel shrink>Task ID</InputLabel>
                                    <Select value={task} sx={{width: '100%'}} onChange={e => updateTask(e.target.value as number)} label="Task ID">
                                        <MenuItem disabled={curItem.id == DeliberationType.data ? false : true} value={-1}><em>{ curItem.id == DeliberationType.data ? "Workflow result": "Select task" }</em></MenuItem>
                                        {taskOptions.map((t, idx) => <MenuItem key={idx} value={idx}>{t.pg[0]}:{t.pg[1]} {t.name}</MenuItem>)}
                                    </Select>
                                </FormControl>
                            ): null
                        }
                        {   curItem.enabled.includes('dataset') ? (
                                <FormControl variant="outlined" sx={{ m: 1, minWidth: 120 }}>
                                    <InputLabel shrink>Dataset ID</InputLabel>
                                    <Select displayEmpty value={dataset} sx={{width: '100%'}} onChange={e => setDataset(e.target.value as string)} label="Dataset ID">
                                        <MenuItem disabled value={''}><em>{ task > -1 ? "Select dataset" : "Select workflow result"}</em></MenuItem>
                                        {  
                                            taskOptions.length && task > -1 ? (
                                                taskOptions[task].datasets.map((t, idx) => <MenuItem key={idx} value={t}>{t}</MenuItem>)
                                            ) : (
                                                workflowResults.map((t, idx) => <MenuItem key={idx} value={t}>{t}</MenuItem>)
                                            )
                                        }
                                    </Select>
                                </FormControl>
                            ) : null
                        }
                        <Button disabled={!canExecute} size="large" variant="contained" sx={{width: '100%'}} onClick={async () => {
                            setLoading(true)
                            try {
                                await exec()
                            } catch(err) {
                                handleErr(err, setResponse)
                            }
                            setLoading(false)
                        }}>Execute</Button>
                    </Stack>
                </div>
                <div style={{flex:1, display: 'flex', maxWidth: 'calc(100% - 225px)'}}>
                    <div style={{width: `calc(${workflowWidth}%)`}}>
                        <FormLabel sx={{marginY: 2,display: 'block'}}>Workflow</FormLabel>
                        <CodeMirror 
                            width="100%"
                            theme="dark"
                            onChange={data => updateWorkflow(data, false)}
                            onBlur={() => {updateWorkflow(workflow, true)}}
                            value={workflow}
                            extensions={[javascript({ jsx: true })]}
                            height="calc(100vh - 235px)"
                            style={{fontSize: '0.9rem'}}
                        />
                    </div>
                    <div ref={contentRef} onMouseDown={handleMouseDown} style={{padding: 10, cursor: 'col-resize'}}></div>
                    <div style={{width: `calc(${responseWidth}%)`}}>
                        <FormLabel sx={{marginY: 2,display: 'block'}}>Response</FormLabel>
                        <CodeMirror
                            basicSetup={{lineNumbers: false}}
                            readOnly
                            width="100%"
                            theme="dark"
                            value={response}
                            extensions={[javascript({ jsx: true }), EditorView.lineWrapping]}
                            height="calc(100vh - 235px)"
                            style={{fontSize: '0.9rem'}}
                        />
                    </div>
                </div>

            </div>
        </div>
        <Login api={API.DELIBERATION}/>
        <Dialog
            open={!!shownError}
            onClose={dismissError}
            aria-labelledby="alert-dialog-title"
            aria-describedby="alert-dialog-description"
        >
            <DialogTitle id="alert-dialog-title">
            {"An error occured"}
            </DialogTitle>
            <DialogContent>
            <DialogContentText id="alert-dialog-description">
                <pre style={{whiteSpace: 'pre-wrap'}}>{errors[0]}</pre>
            </DialogContentText>
            </DialogContent>
            <DialogActions>
            <Button onClick={dismissError}>Ok</Button>
            </DialogActions>
        </Dialog>
        </>
    )
}


export default DeliberationPage