import * as React from 'react';
import CircularProgress from '@mui/material/CircularProgress';
import Box from '@mui/material/Box';

export const Loader = () => {
    return (
        <Box sx={{
            display: 'flex', top: 0, left: 0, position: 'fixed',
            width: '100%', height: '100%', backgroundColor: 'rgba(0,0,0,0.8)',
            zIndex: 9999, justifyContent: 'center', 'alignItems': 'center'
            }}>
            <CircularProgress />
        </Box>
    )
}