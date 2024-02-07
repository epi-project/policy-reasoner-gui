import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"
import { createContext, useEffect, useRef, useState } from "react"
import { authenticate, getAuthData, removeAuth } from "../api"

export enum API {
    POLICY = 'policy',
    DELIBERATION = 'deliberation',
}

export interface AuthState {
    authenticated: (api: API) => boolean
    login: (api: API, jwt: string) => void,
    logout: (api: API) => void
}

export const AuthContext = createContext<AuthState | null>(null)

export const useAuth = () => {
    const client = useQueryClient()
    const [state, setState] = useState({
        [API.POLICY]: false,
        [API.DELIBERATION]: false,
    })

    const {data: authData} = useQuery({
        queryKey: ['auth'],
        queryFn: getAuthData
    })

    const login= useMutation({
        mutationFn: authenticate,
        onError: (err) => {
            console.error(err)
        },
        onSuccess: () => {
            client.invalidateQueries({queryKey:['auth']})
        }
    })

    const logout = useMutation({
        mutationFn: removeAuth,
        onError: (err) => {
            console.error(err)
        },
        onSuccess: () => {
            client.invalidateQueries({queryKey:['auth']})
        }
    })

    useEffect(() => {
        setState({
            [API.POLICY]: !!authData?.policy,
            [API.DELIBERATION]: !!authData?.deliberation,
        })
    }, [authData])

    return {
        authenticated: (api: API) => {
            return state[api]
        },
        login: (api: API, jwt: string) => {
            login.mutate({api, token: jwt})
        },
        logout: () => {
            logout.mutate()
        }
    }
}