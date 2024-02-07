import React, { FC, useContext, useState } from 'react'
import { API, AuthContext } from '../context/auth'

import { TransitionProps } from '@mui/material/transitions';
import { Slide, Dialog, DialogTitle, DialogContent, DialogContentText, DialogActions, Button, TextField } from '@mui/material';

const Transition = React.forwardRef(function Transition(
  props: TransitionProps & {
    children: React.ReactElement<any, any>;
  },
  ref: React.Ref<unknown>,
) {
  return <Slide direction="up" ref={ref} {...props} />;
});


export interface LoginProps {
    api: API
}

const Login : FC<LoginProps> = ({api}) => {
    const [jwt, setJwt] = useState('')
    const auth = useContext(AuthContext)

    const login = () => {
        auth?.login(api, jwt)
    }
    return (
        <Dialog
        fullWidth={true}
        open={!auth?.authenticated(api)}
        TransitionComponent={Transition}
        keepMounted
        
        onClose={() => {}}
        aria-describedby="alert-dialog-slide-description"
      >
        <DialogTitle>{`Please authenticate to use the ${api} API`}</DialogTitle>
        <DialogContent>
          <TextField minRows={5} fullWidth={true} value={jwt} onChange={x => setJwt(x.target.value)} multiline placeholder='paste JWT'/>
        </DialogContent>
        <DialogActions>
          <Button disabled={!jwt} onClick={login}>Login</Button>
        </DialogActions>
      </Dialog>
    )
}

export default Login