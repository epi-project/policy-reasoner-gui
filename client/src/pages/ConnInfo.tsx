import CodeMirror from '@uiw/react-codemirror';
import { javascript } from '@codemirror/lang-javascript';
import { Button, Dialog, DialogActions, DialogContent, DialogContentText, DialogTitle, FormControl, FormLabel, MenuItem, Select } from "@mui/material"
import React, { FC, useContext, useEffect, useState } from "react"
import { API, AuthContext, AuthState } from '../context/auth';
import Login from '../components/Login';
import useConnectorInfo from '../hooks/useConnectorInfo';
import { SYNTAX } from '../api/types';
import { convEFlintJSON } from '../api';
import { Loader } from '../components/Loader';
import { handleError } from '../errors/util';


const ConnInfo: FC = () => {
    const authData = useContext(AuthContext)
    const [mode, setMode] = useState<SYNTAX>(SYNTAX.EFLINT)
    const [eflintBase, setEFlintBase] = useState('')
    const [errors, setErrors] = useState<string[]>([])
    const [shownError, setShownError] = useState<string>()

    const {reasonerConnectorError, reasonerConnectorInfo: info, reasonerConnectorIsFetching} = useConnectorInfo(authData)
    const isLoading = authData?.authenticated(API.POLICY) && (reasonerConnectorIsFetching)
    
    const dismissError = () => {
        setShownError(undefined)
        setTimeout(() => setErrors(x => x.slice(1)), 500)
    }

    useEffect(() => {
        if (!errors.length || !!shownError) {
            return
        }

        setShownError(errors[0])
    }, [errors])
    
    useEffect(() => {
        (async () => {
            if (!info || reasonerConnectorError) {
                return
            }
            const eflintBaseDefs = await convEFlintJSON(info?.context.base_defs as string)

            setEFlintBase(eflintBaseDefs)
        })()
    }, [info])

    useEffect(() => {
        handleError(reasonerConnectorError, setErrors, authData!)
    }, [reasonerConnectorError])

    return (
        <>
        <div>
            {!errors.length && isLoading ? <Loader/> : null}
            <h1>Reasoner connector info</h1>
            <div style={{display: 'flex'}}>
                <div style={{marginRight: 25}}>
                    <FormLabel sx={{marginY: 2}}>Type</FormLabel>
                    <p>{info ? info.context.type : ''}</p>
                </div>
                <div style={{marginRight: 25}}>
                    <FormLabel sx={{marginY: 2}}>Version</FormLabel>
                    <p>{info ? info.context.version : ''}</p>
                </div>
                <div>
                    <FormLabel sx={{marginY: 2}}>Hash</FormLabel>
                    <p>{info ? info.hash : ''}</p>
                </div>
            </div>
            <div style={{marginTop: 20, position: 'relative'}}>
                <div style={{display: 'flex'}}>
                    <FormLabel sx={{marginY: 2,display: 'block'}}>Base definitions</FormLabel>
                    <FormControl size='small' variant="outlined" sx={{ m: 1, minWidth: 130}}>
                        <Select value={mode} sx={{width: '100%'}} onChange={e => setMode(e.target.value as SYNTAX)}>
                            <MenuItem value={SYNTAX.EFLINT}>E-Flint</MenuItem>
                            <MenuItem value={SYNTAX.JSON}>E-Flint json</MenuItem>
                        </Select>
                    </FormControl>
                </div>
                <CodeMirror
                    theme="dark"
                    readOnly
                    value={mode == SYNTAX.JSON ? info?.context.base_defs as string : eflintBase}
                    extensions={[javascript({ jsx: true })]}
                    />
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


export default ConnInfo
