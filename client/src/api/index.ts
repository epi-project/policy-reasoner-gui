import axios from "axios"
import { API } from "../context/auth";
import { Policy, PolicyVersion, reasonerConnectorInfo, AuthDataViewmodel, Workflow, DeliberationType, Option, WorkflowConvResult } from "./types";
import { helloWorldPolicy } from "./hello-world-example";


axios.defaults.withCredentials = true;

const baseAddr = process.env.BACKEND_ADDR ? process.env.BACKEND_ADDR : 'http://localhost:3001/api'

const buildUrl = (...path : string[]) : string => {
    return [baseAddr, ...path].join('/')
}

export const NEW_VERSION = -1

export const newPolicy = (parent: Policy | null) : Policy => ({
    version: -1,
    reasoner: '',
    reasonerVersion: '',
    content: parent ? parent.content : helloWorldPolicy,
    jsonContent: '',
    description: '',
    versionDescription: ''
})

export const getPolicies = async () : Promise<PolicyVersion[]> => {
    return (await axios.get(buildUrl('policies'))).data as PolicyVersion[]
}

export const activateVersion = async (version: number)  => {
    return await axios.post(buildUrl('policies', 'active'), {
        version
    })
}

export const deactivateVersion = async ()  => {
    return await axios.delete(buildUrl('policies', 'active'))
}

export const getReasonerConnectorInfo = async (): Promise<reasonerConnectorInfo> =>  {
    return (await axios.get(buildUrl('reasoner-connector-info'))).data as reasonerConnectorInfo
}

export const addPolicy = async (content: string, version_description: string, info: reasonerConnectorInfo)  => {
    const jsonContent = await convEFlint(content)

    return await axios.post(buildUrl('policies'), {
        description: '',
        version_description,
        content: [{
            reasoner: info.context.type,
            reasoner_version: info.context.version,
            content: jsonContent ,
        }]
    })
}

export const getActiveVersion = async () : Promise<number> => {
    try {
        const result = await axios.get(buildUrl('policies', 'active'))
        return result.data.version as number
    } catch(err) {
        console.error(err)
        return 0
    }
}

export const getPolicy = async (version?: number) : Promise<Policy | null> => {
    if (!version || version === NEW_VERSION) {
        return null
    }

    const policy = (await axios.get(buildUrl('policies', version.toString(10)))).data as any

    const eflintPolicy = await convEFlintJSON(policy.content[0].content)

    return {
        version: policy.version,
        reasoner: policy.content[0].reasoner,
        reasonerVersion: policy.content[0].reasoner_version,
        description: policy.description,
        versionDescription: policy.version_description,
        content: eflintPolicy,
        jsonContent: JSON.stringify(policy.content[0].content, null, '    ')
    }
}

export const getAuthData = async () : Promise<AuthDataViewmodel> => {
    return (await axios.get(buildUrl('authenticate'))).data as AuthDataViewmodel
}

export const authenticate = async ({api, token}: {api: API, token: string}) : Promise<undefined> => {
    await axios.post(buildUrl('authenticate'), {t: api, token}, { headers: {'Content-Type': 'application/json'} })
} 

export const removeAuth = async () : Promise<undefined> => {
    await axios.delete(buildUrl('authenticate'))
}

export const convBS = async(bsWorkflow: string) : Promise<WorkflowConvResult> => {
    return (await axios.post(buildUrl('conv') + "?from=branescript&to=wir", bsWorkflow)).data as WorkflowConvResult
}

export const convEFlintJSON = async(eflintJSON: string) : Promise<string> => {
    return (await axios.post(buildUrl('conv') + "?from=eflintjson&to=eflint", eflintJSON)).data
}

export const convEFlint = async(eflint: string) : Promise<string> => {
    return (await axios.post(buildUrl('conv') + "?from=eflint&to=eflintjson", eflint)).data
}

export const extractTasks = (wf: Workflow) : Option[] => {
    let tasks = wf.graph.map((cur, idx) => cur.kind == "nod" ? {
        name: wf.table.tasks[cur.t].d.n,
        pg: ["<main>", idx]
    } : null).filter(Boolean) as Option[]

    tasks = Object.keys(wf.funcs).reduce<Option[]>((acc, fnIdx) => {
        const edges = wf.funcs[parseInt(fnIdx)]

        const tasks = edges.map(
            (e, idx) => e.kind == "nod" ? {name: wf.table.tasks[e.t].d.n, pg: [fnIdx, idx]} : null
        ).filter(Boolean) as Option[]

        return [...acc, ...tasks]
    }, tasks)

    return tasks
}

export const deliberate = async ({type, req}:  {type: DeliberationType, req: any})  => {
    req.workflow.user = 'test'
    const body = JSON.stringify(req).replace('18446744073709552000', '18446744073709551615')

    return await axios.post(buildUrl('deliberation', type), body, { headers: {'Content-Type': 'application/json'} })
}