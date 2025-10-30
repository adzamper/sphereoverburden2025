import React, { useState, useCallback } from 'react';
import { Box, Container, Grid, Paper, ThemeProvider, createTheme } from '@mui/material';
import ParameterInputs from './components/ParameterInputs';
import ResponsePlots from './components/ResponsePlots';
import { CalculationParams, ResponseData } from './types';
import axios from 'axios';

const theme = createTheme({
  typography: {
    fontFamily: "'Inter', sans-serif",
    h6: {
      fontWeight: 600,
    },
    subtitle1: {
      fontWeight: 500,
    },
  },
  palette: {
    primary: {
      main: '#6366f1',
    },
    background: {
      default: '#f8fafc',
      paper: '#ffffff',
    },
  },
  components: {
    MuiPaper: {
      styleOverrides: {
        root: {
          borderRadius: 16,
          boxShadow: '0 4px 20px rgba(0,0,0,0.05)',
        },
      },
    },
    MuiTextField: {
      styleOverrides: {
        root: {
          '& .MuiOutlinedInput-root': {
            borderRadius: 12,
            backgroundColor: '#ffffff',
            fontSize: '0.9rem',
            transition: 'all 0.2s',
            '&:hover': {
              backgroundColor: '#fafafa',
            },
          },
          '& label': {
            fontSize: '0.9rem',
          },
        },
      },
    },
    MuiButton: {
      styleOverrides: {
        root: {
          borderRadius: 12,
          boxShadow: 'none',
          '&:hover': {
            boxShadow: '0 4px 12px rgba(99, 102, 241, 0.2)',
          },
        },
      },
    },
  },
});

const defaultParams: CalculationParams = {
  radar: 120,
  mu: 1.2566370614359172e-6,
  dipole_m: 1847300,
  rtxrx: { x: 12.5, y: 0, z: 56 },
  rsp: { x: 0, y: 0, z: -200 },
  a: 100,
  sigma_sp: 2.5,
  mtx: { x: 0, y: 0, z: 1 },
  sigma_ob: 0.03333333333333333,
  thick_ob: 15,
  apply_dip: false,
  strike: 90,
  dip: 90,
  base_freq: 25,
  period: 0.04,
  pulse_length: 0.00365,
  xsign_negative: false,
  profile_length: 800,
};

function App() {
  const [params, setParams] = useState<CalculationParams>(defaultParams);
  const [response, setResponse] = useState<ResponseData | null>(null);
  const [loading, setLoading] = useState(false);

  const handleCalculate = useCallback(async () => {
    setLoading(true);
    try {
      const result = await axios.post('/api/calculate', params);
      setResponse(result.data);
    } catch (error) {
      console.error('Error calculating response:', error);
    } finally {
      setLoading(false);
    }
  }, [params]);

  return (
    <ThemeProvider theme={theme}>
      <Box sx={{ 
        minHeight: '100vh',
        backgroundColor: 'background.default',
        py: 2,
        background: 'linear-gradient(45deg, #f0f2f5 0%, #e3f2fd 100%)',
      }}>
        <Container maxWidth={false} sx={{ maxWidth: '1800px' }}>
          <Grid container spacing={2}>
            <Grid item xs={12} md={3}>
              <Paper 
                elevation={3} 
                sx={{ 
                  height: 'calc(100vh - 32px)',
                  backgroundColor: 'background.paper',
                  overflow: 'hidden',
                }}
              >
                <ParameterInputs 
                  params={params} 
                  setParams={setParams}
                  onCalculate={handleCalculate}
                  loading={loading}
                />
              </Paper>
            </Grid>
            <Grid item xs={12} md={9}>
              <Paper 
                elevation={3} 
                sx={{ 
                  p: 3,
                  backgroundColor: 'background.paper',
                }}
              >
                <ResponsePlots response={response} />
              </Paper>
            </Grid>
          </Grid>
        </Container>
      </Box>
    </ThemeProvider>
  );
}

export default React.memo(App); 
