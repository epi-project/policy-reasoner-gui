import React, { FC, useContext, useEffect, useState } from "react"
import { AuthContext, API } from "../context/auth"
import Login from "../components/Login"
import { Alert, Button, ButtonGroup, Dialog, DialogActions, DialogContent, DialogContentText, DialogTitle, FormControl, Input, MenuItem, MenuList, TextField, ToggleButton, ToggleButtonGroup } from "@mui/material"
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"
import { NEW_VERSION, Policy, PolicyVersion, activateVersion, addPolicy, deactivateVersion, getActiveVersion, getPolicies, getPolicy, newPolicy } from "../api"
import CodeMirror, { EditorView } from '@uiw/react-codemirror';
import { javascript } from '@codemirror/lang-javascript';
import { AxiosError } from "axios"
import useConnectorInfo from "../hooks/useConnectorInfo"

const handleError = (error: any, setErrors: React.Dispatch<React.SetStateAction<string[]>>) => {
    if (!error) {
        return
    }

    const response = (error as AxiosError).response

    if (!response) {
        let msg = (error as AxiosError).message
        setErrors(e => [...e, msg || `${msg}`])
        return
    }

    const err = (response.data as any).detail

    if (!err) {
        setErrors(e => [...e, `Call returned invalid statuscode: ${response.status} (${response.statusText})`])
        return
    }

    setErrors(e => [...e, err])
}

const PoliciesPage: FC = () => {
    const authData = useContext(AuthContext)
    const client = useQueryClient()
    const [statePolicies, setStatePolicies] = useState<PolicyVersion[]>([])
    const [selected, setSelected] = useState<number>()
    const [editPolicy, setEditPolicy] = useState<Policy>()
    const [errors, setErrors] = useState<string[]>([])
    const [shownError, setShownError] = useState<string>()
    const {reasonerConnectorError, reasonerConnectorInfo, reasonerConnectorIsFetching} = useConnectorInfo(authData)


    const {isPending, error, data: policies, isFetching} = useQuery({
        queryKey: ['policies'],
        queryFn: getPolicies
    })

    const {isPending: policyIsPending, error: policyError, data: policy, isFetching: policyIsFetching} = useQuery({
        queryKey: ['policies', selected],
        queryFn: () => getPolicy(selected)
    })

    const {data: activeVersion} = useQuery({
        queryKey: ['policies', 'active'],
        queryFn: getActiveVersion
    })

    const activateMutation = useMutation({
        mutationFn: activateVersion,
        onSuccess: () => {
            // Invalidate and refetch
            client.invalidateQueries({ queryKey: ['policies', 'active'] })
        },
        onError: (error) => handleError(error, setErrors)
    })

    const deactivateMutation = useMutation({
        mutationFn: deactivateVersion,
        onSuccess: () => {
            // Invalidate and refetch
            client.invalidateQueries({ queryKey: ['policies', 'active'] })
        },
        onError: (error) => handleError(error, setErrors)
    })

    const addPolicyMutation = useMutation({
        mutationFn: (policy: Policy) => addPolicy(JSON.parse(policy.content), policy.versionDescription, reasonerConnectorInfo!),
        onSuccess: () => {
            // Invalidate and refetch
            setStatePolicies(x => x.filter(p => p.version !== NEW_VERSION))
            setSelected(0)
            setEditPolicy(undefined)
            client.invalidateQueries({ queryKey: ['policies'] })
        },
        onError: (error) => handleError(error, setErrors)
    })

    const staticVersion = (!policy || policy.version > 0) && selected !== NEW_VERSION

    const addNewPolicy = () => {
        setStatePolicies([{version: -1}, ...statePolicies])
        setEditPolicy(newPolicy(policy ? policy : null))
        setSelected(NEW_VERSION)
    }

    const dismissError = () => {
        setShownError(undefined)
        setTimeout(() => setErrors(x => x.slice(1)), 500)
        
    }

    useEffect(() => {
        const newPolicy = statePolicies.filter(x => x.version === NEW_VERSION)[0]
        const p = [newPolicy, ...(policies || [])].filter(Boolean)
        setStatePolicies(p)
        p.length && setSelected(p[0].version)
    }, [policies])

    useEffect(() => {
        if (!authData?.authenticated(API.POLICY) || !error) {
            return
        }
        client.invalidateQueries({ queryKey: ['policies'] })
        client.invalidateQueries({ queryKey: ['policies', 'active'] })
    }, [authData?.authenticated(API.POLICY), error])

    useEffect(() => {
        if (!errors.length || !!shownError) {
            return
        }

        setShownError(errors[0])
    }, [errors])

    const loading = policyIsFetching || isFetching || activateMutation.isPending

    const commitVersion = () => {
        addPolicyMutation.mutate(editPolicy!)
    }

    return (
        <>
        <div>
            <h1>Policy API</h1>
            <div style={{display: 'flex', justifyContent: 'space-between'}}>
                
                <div style={{width: '300px', marginRight: 10, paddingTop: 5}}>
                    <div style={{display: 'flex', height: 55}}>
                        <Button
                            onClick={addNewPolicy}
                            disabled={selected === NEW_VERSION}
                            style={{position: "relative", top: 0.5, flex: 1, marginRight: 5}}
                            size="large"
                            variant="outlined"
                        >
                            New
                        </Button>
                        <Button
                            style={{position: "relative", top: 0.5, flex: 1, marginLeft: 5}} 
                            size="large"
                            variant="outlined"
                            disabled={!selected || selected === NEW_VERSION || loading}
                            onClick={() => selected === activeVersion ? deactivateMutation.mutate() : activateMutation.mutate(selected!)}
                        >
                            {  selected === activeVersion ? 'Deactivate' : 'Activate' }
                        </Button>
                    </div>
                    <div style={{marginTop: 10}}>
                        <ToggleButtonGroup orientation="vertical" value={selected} size="large" fullWidth>
                            {
                                authData?.authenticated(API.POLICY) && statePolicies.length ? statePolicies.map(
                                    x => (
                                        <ToggleButton key={x.version} color={activeVersion === x.version ? 'success' : 'standard'} onClick={() => setSelected(x.version)} value={x.version}>
                                            Version {x.version === NEW_VERSION ? 'New' : x.version} {activeVersion === x.version ? ' (active)' : ''}
                                        </ToggleButton>
                                    )
                                ) : null
                            }
                        </ToggleButtonGroup>
                    </div>
                </div>
                <div style={{flex: 1, overflowX: 'hidden', paddingTop: 5}}>
                    <div style={{display: 'flex'}}>
                        <TextField
                            disabled={staticVersion}
                            value={staticVersion ? policy?.versionDescription || '' : editPolicy?.versionDescription}
                            onChange={(e) => setEditPolicy({...editPolicy!, versionDescription: e.target.value})}
                            style={{flex: 1, marginRight: 10}}
                            multiline
                            maxRows={1}
                            label="Description / commmit message"/>
                        <Button variant="outlined" disabled={staticVersion} onClick={commitVersion}>Commit</Button>
                    </div>
                    <div style={{marginTop: 10}}>
                        <CodeMirror
                            theme="dark"
                            readOnly={staticVersion}
                            value={staticVersion ? (policy ? policy.content : '') : editPolicy!.content}
                            onChange={e => setEditPolicy({...editPolicy!, content: e})}
                            extensions={[javascript({ jsx: true }), EditorView.lineWrapping]}
                            maxWidth="100%" height="calc(100vh - 245px)"/>
                    </div>
                </div>
            </div>
        </div>
        <Login api={API.POLICY}/>
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
                {errors[0]}
            </DialogContentText>
            </DialogContent>
            <DialogActions>
            <Button onClick={dismissError}>Ok</Button>
            </DialogActions>
        </Dialog>
        </>
    )
}


export default PoliciesPage