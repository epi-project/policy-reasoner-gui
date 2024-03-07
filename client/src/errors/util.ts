import { AxiosError } from "axios"
import { AuthState, API } from "../context/auth"

export const handleError = (error: any, setErrors: React.Dispatch<React.SetStateAction<string[]>>, authData: AuthState) => {
    if (!error) {
        return
    }

    const response = (error as AxiosError).response

    if (!response) {
        let msg = (error as AxiosError).message
        setErrors(e => [...e, msg || `${msg}`])
        return
    }

    if (response.status == 401) {
        // don't report authentication error but logout
        authData.logout(API.POLICY)
        return
    } 

    const err = (response.data as any).detail || response.data as string

    if (!err) {
        setErrors(e => [...e, `Call returned invalid statuscode: ${response.status} (${response.statusText})`])
        return
    }

    setErrors(e => [...e, err])
}