import CodeMirror from '@uiw/react-codemirror';
import { javascript } from '@codemirror/lang-javascript';
import { FormControl, FormLabel, MenuItem, Select } from "@mui/material"
import React, { FC, useContext, useEffect, useState } from "react"
import { API, AuthContext } from '../context/auth';
import Login from '../components/Login';
import useConnectorInfo from '../hooks/useConnectorInfo';
import { SYNTAX } from '../api/types';
import { convEFlintJSON } from '../api';

const ConnInfo: FC = () => {
    const authData = useContext(AuthContext)
    const [mode, setMode] = useState<SYNTAX>(SYNTAX.EFLINT)
    const [eflintBase, setEFlintBase] = useState('')
    
    const {reasonerConnectorError, reasonerConnectorInfo: info, reasonerConnectorIsFetching} = useConnectorInfo(authData)

    useEffect(() => {
        (async () => {
            console.log(info, reasonerConnectorError)
            if (!info || reasonerConnectorError) {
                return
            }
            const eflintBaseDefs = await convEFlintJSON(info?.context.base_defs as string)

            setEFlintBase(eflintBaseDefs)
        })()
    }, [info])

    return (
        <>
        <div>
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
        </>
    )
}


export default ConnInfo