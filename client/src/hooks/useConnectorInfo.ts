import { useQuery, useQueryClient } from "@tanstack/react-query"
import { useEffect } from "react"
import { getReasonerConnectorInfo } from "../api"
import { API, AuthState } from "../context/auth"

const useConnectorInfo = (authData: AuthState | null) => {
    const client = useQueryClient()
    
    const {isPending, error, data: reasonerConnectorInfo, isFetching} = useQuery({
        queryKey: ['connector-context'],
        queryFn: getReasonerConnectorInfo
    })

    useEffect(() => {
        if (!authData?.authenticated(API.POLICY) || !error) {
            return
        }
        client.invalidateQueries({ queryKey: ['connector-context'] })
    }, [authData?.authenticated(API.POLICY), error])

    return {reasonerConnectorError: error, reasonerConnectorInfo, reasonerConnectorIsFetching: isFetching}
}

export default useConnectorInfo