import React, { FC, useRef, useState } from "react"
import { Button, FormLabel, MenuItem, Select, Stack, TextField } from "@mui/material"
import { javascript } from "@codemirror/lang-javascript";
import CodeMirror, { EditorView } from '@uiw/react-codemirror';
import Login from "../components/Login";
import { API } from "../context/auth";
import { DeliberationType, Workflow, convBS, deliberate, extractTasks, Option } from "../api";
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

const DeliberationPage: FC = () => {
    const [curItem, setCurItem] = useState<MenuItemData>(menuItems[0])
    const [loading, setLoading] = useState<boolean>(false)
    const [task, setTask] = useState<number>(-1)
    const [dataset, setDataset] = useState('')
    const [workflow, setWorkflow] = useState('')
    const [jsonWorkflow, setJsonWorkflow] = useState<Workflow | null>(null)
    const [taskOptions, setTaskOptions] = useState<Option[]>([])
    const [response, setResponse] = useState('')
    const [workflowWidth, setWorkflowWidth] = useState(60)
    const [responseWidth, setResponseWidth] = useState(40)
    const contentRef = useRef(null)


    const updateWorkflow = async (wf: string) => {
        if (!wf) {
            setWorkflow(wf)
            setJsonWorkflow(null)
            setTaskOptions([])
            return
        }
        const jsonwf = await convBS(wf)
        const options = extractTasks(jsonwf)

        setWorkflow(wf)
        setJsonWorkflow(jsonwf)
        setTaskOptions(options)
        setTask(-1)
    }

    // extractTasks()

    const handleErr = (err: any) => {
        setResponse(err.message)
    }

    const canExecute = validate(curItem, task != -1 ? taskOptions[task] : undefined, dataset, workflow)
    const mutation = useMutation({
        mutationFn: deliberate,
        onSuccess: (r) => {
            console.log(r)
        },
        onError: (error) => {
            if (!error) {
                return
            }
        
            const response = (error as AxiosError).response
        
            if (!response) {
                let msg = (error as AxiosError).message
                setResponse(msg || `${msg}`)
                return
            }

            let err = (response.data as any).detail || response.data
        
            if (!err) {
                setResponse(`Call returned invalid statuscode: ${response.status} (${response.statusText})`)
                return
            }

            err = `Call returned invalid statuscode: ${response.status} (${response.statusText})

${err}            
` 
        
            setResponse(err)
        }
    })
    const exec = async  () => {
        const req = await curItem.generateRequest(jsonWorkflow!, task != -1 ? taskOptions[task] : undefined)
        
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

    console.log('tasks', taskOptions)

    return (
        <>
        <div>
            <h1>Deliberation API</h1>
            <div style={{display: 'flex', justifyContent: 'space-between'}}>
                
                <div style={{width: '200px', paddingTop: 50}}>
                    <Stack spacing={1}>
                        <Select
                            sx={{width: '100%'}}
                            value={curItem.id}
                            onChange={e => {
                                setTask(-1)
                                setDataset('')
                                setCurItem(menuItems.filter(item => item.id === e.target.value)[0])
                            }}
                        >
                            {menuItems.map(item => <MenuItem key={item.id} value={item.id}>{item.label}</MenuItem>)}
                        </Select>
                        {
                            curItem.enabled.includes('task') ? (
                                <Select value={task} sx={{width: '100%'}} onChange={e => setTask(e.target.value as number)} label="Task ID">
                                    <MenuItem value={-1}><em>Select task</em></MenuItem>
                                    {taskOptions.map((t, idx) => <MenuItem key={idx} value={idx}>{t.pg[0]}:{t.pg[1]} {t.name}</MenuItem>)}
                                </Select>
                            ): null
                        }
                        {curItem.enabled.includes('dataset') ? <TextField value={dataset} sx={{width: '100%'}} onChange={e => setDataset(e.target.value as string)} label="Dataset ID"/> : null}
                        <Button disabled={!canExecute} size="large" variant="contained" sx={{width: '100%'}} onClick={async () => {
                            setLoading(true)
                            try {
                                await exec()
                            } catch(err) {
                                console.log('errrr', err)
                                handleErr(err)
                            }
                            setLoading(false)
                        }}>Execute</Button>
                    </Stack>
                </div>
                <div style={{flex:1, display: 'flex'}}>
                    <div style={{width: `${workflowWidth}%`, padding: "0 0 0 25px"}}>
                        <FormLabel sx={{marginY: 2,display: 'block'}}>Workflow</FormLabel>
                        <CodeMirror width="100%" theme="dark" onChange={data => updateWorkflow(data)} value={workflow} extensions={[javascript({ jsx: true })]} height="calc(100vh - 235px)"/>
                    </div>
                    <div ref={contentRef} onMouseDown={handleMouseDown} style={{padding: 10, cursor: 'col-resize'}}></div>
                    <div style={{width: `${responseWidth}%`}}>
                        <FormLabel sx={{marginY: 2,display: 'block'}}>Response</FormLabel>
                        <CodeMirror
                            basicSetup={{lineNumbers: false}}
                            readOnly
                            width="100%"
                            theme="dark"
                            value={response}
                            extensions={[javascript({ jsx: true }), EditorView.lineWrapping]}
                            height="calc(100vh - 235px)"
                        />
                    </div>
                </div>

            </div>
        </div>
        <Login api={API.DELIBERATION}/>
        </>
    )
}


export default DeliberationPage