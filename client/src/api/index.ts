import axios from "axios"
import { API } from "../context/auth";


axios.defaults.withCredentials = true;

const baseAddr = 'http://localhost:3001'

const buildUrl = (...path : string[]) : string => {
    return [baseAddr, ...path].join('/')
}

export interface PolicyVersion {
    version: number;
}

export const NEW_VERSION = -1

const template = `[
    {
        "version": "0.1.0",
        "kind": "phrases",
        "phrases": []
    }
]`

export const newPolicy = (parent: Policy | null) : Policy => ({
    version: -1,
    reasoner: '',
    reasonerVersion: '',
    content: parent ? parent.content : template,
    description: '',
    versionDescription: ''
})

export interface Policy {
    version: number;
    reasoner: string;
    reasonerVersion: string;
    content: string;
    description: string;
    versionDescription: string;
}

interface PolicyContentPostModel {
    reasoner: string;
    reasoner_version: string;
    content: any;
}

interface PostPolicy {
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
    return await axios.post(buildUrl('policies'), {
        description: '',
        version_description,
        content: [{
            reasoner: info.context.type,
            reasoner_version: info.context.version,
            content,
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

    return {
        version: policy.version,
        reasoner: policy.content[0].reasoner,
        reasonerVersion: policy.content[0].reasoner_version,
        description: policy.description,
        versionDescription: policy.version_description,
        content: JSON.stringify(policy.content[0].content, null, 4)
    }
}

export interface AuthDataViewmodel{
    policy: string,
    deliberation: string
}

export interface AuthDataPostModel{
    t: API,
    token: string,
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

export enum DeliberationType {
    'task' = 'task',
    'data' = 'data',
    'workflow' = 'workflow',
}

interface Edge {
    "kind": string,
    "t": number,
}

interface Task {
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

type ProgramCounter = ["<main>" | number, number]

export interface Option {
    name: string,
    pg: ProgramCounter
}

export const convBS = async(bsWorkflow: string) : Promise<Workflow> => {
    return (await axios.post(buildUrl('conv') + "?from=branescript&to=wir", bsWorkflow)).data as Workflow
}

export const extractTasks = (wf: Workflow) : Option[] => {
    let tasks = wf.graph.map((cur, idx) => cur.kind == "nod" ? {
        name: wf.table.tasks[cur.t].d.n,
        pg: ["<main>", idx]
    } : null).filter(Boolean) as Option[]

    console.log('wf.graph', wf.graph)

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
    const body = JSON.stringify(req).replace('18446744073709552000', '18446744073709551615')

    return await axios.post(buildUrl('deliberation', type), body, { headers: {'Content-Type': 'application/json'} })
}