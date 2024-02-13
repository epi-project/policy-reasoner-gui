import {
  AppBar,
  Box,
  Button,
  ButtonGroup,
  Container,
  Menu,
  MenuItem,
  Typography,
} from "@mui/material";
import React, { FC, useContext } from "react";
import { Link, Outlet, useLocation, useMatch, useNavigate } from "react-router-dom";
import { API, AuthContext } from "../context/auth";

const Layout: FC = () => {
  const authData = useContext(AuthContext)
  const navigate = useNavigate()
  const isPolicyRoute = useMatch("/policies")
  const isDeliberationRoute = useMatch("/deliberation")
  const isIndexRoute = useMatch("/")
  const isConnInfoRoute = useMatch("/reasoner-connector-info")
    
  return (
    <div>
      <AppBar position="fixed" sx={{zIndex: 9999}}>
        <div style={{padding: '0 25px'}}>
          <Box sx={{ flexGrow: 1, display: { xs: 'none', md: 'flex'} }} style={{justifyContent: 'space-between'}}>
                <div style={{display: 'flex'}}>
                <Button onClick={() => navigate("/reasoner-connector-info")} sx={{ my: 2, color: isIndexRoute || isConnInfoRoute ? '#4dabf5' : 'white', display: 'block' }}>
                      Reasoner connector info
                </Button>
                <Button onClick={() => navigate("/policies")} sx={{ my: 2, color: isPolicyRoute ? '#4dabf5' : 'white', display: 'block' }}>
                      Policies
                </Button>
                <Button onClick={() => navigate("/deliberation")} sx={{ my: 2, color: isDeliberationRoute ? '#4dabf5' : 'white', display: 'block' }}>
                      Deliberation
                </Button>
                </div>
               
                {authData?.authenticated(API.DELIBERATION) || authData?.authenticated(API.POLICY) ? (
                   <Button
                      onClick={async () => {
                      if (authData.authenticated(API.DELIBERATION)) {
                        await authData.logout(API.DELIBERATION)
                      }
                      if (authData.authenticated(API.POLICY)) {
                        await authData.logout(API.POLICY)
                      }
                    }} sx={{ my: 2, color: 'white', display: 'block' }}>
                      Logout
                  </Button>
                ) : null}
            </Box>
          </div>
      </AppBar> 
      <div style={{padding: '58px 25px 0 25px'}}>
        <Box sx={{ flexGrow: 1, p: 1 }}>
          <Outlet />
        </Box>
      </div>
    </div>
  );
};

export default Layout;
