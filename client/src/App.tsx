import React, { FC } from 'react';
import {
    createBrowserRouter,
    RouterProvider
} from "react-router-dom";
import {
    QueryClient,
    QueryClientProvider,
} from '@tanstack/react-query';


import Layout from './components/Layout';
import PoliciesPage from './pages/Policies';
import ConnInfo from './pages/ConnInfo';
import DeliberationPage from './pages/Deliberation';
import { CssBaseline, ThemeProvider, createTheme } from '@mui/material';
import { AuthContext, useAuth } from './context/auth';


const router = createBrowserRouter([
{
    path: "/",
    element: <Layout/>,
    children: [{
        index: true,
        path: "/",
        element: <ConnInfo/>
    }, {
        path: "/policies",
        element: <PoliciesPage/>
    }, {
        path: "/deliberation",
        element: <DeliberationPage/>
    }, {
        path: "/reasoner-connector-info",
        element: <ConnInfo/>
    }]
},
]);


const darkTheme = createTheme({
    palette: {
      mode: 'dark',
    },
  });

// Create a client
const queryClient = new QueryClient()


const AuthenticatedApp = () => {
    const auth = useAuth()

    return  (
        <AuthContext.Provider value={auth}>
            <ThemeProvider theme={darkTheme}>
                <CssBaseline />
                <RouterProvider router={router} />
            </ThemeProvider>
        </AuthContext.Provider>
    )
}
  

const App: FC = () => {
    return  (
        <QueryClientProvider client={queryClient}>
            <AuthenticatedApp/>
        </QueryClientProvider>
    )
}

export default App