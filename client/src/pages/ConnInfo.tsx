import CodeMirror from '@uiw/react-codemirror';
import { javascript } from '@codemirror/lang-javascript';
import { FormLabel } from "@mui/material"
import React, { FC, useContext } from "react"
import { API, AuthContext } from '../context/auth';
import Login from '../components/Login';
import useConnectorInfo from '../hooks/useConnectorInfo';

const ConnInfo: FC = () => {
    const authData = useContext(AuthContext)
    
    const {reasonerConnectorError, reasonerConnectorInfo: info, reasonerConnectorIsFetching} = useConnectorInfo(authData)

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
            <div style={{marginTop: 20}}>
                <FormLabel sx={{marginY: 2,display: 'block'}}>Base definitions</FormLabel>
                <CodeMirror
                    theme="dark"
                    readOnly
                    value={info?.context.base_defs as string}
                    extensions={[javascript({ jsx: true })]}
                    height="calc(100vh - 315px)"/>
            </div>
        </div>
        <Login api={API.POLICY}/>
        </>
    )
}


export default ConnInfo