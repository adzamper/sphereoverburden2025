import React from 'react';
import {
  Box,
  TextField,
  Typography,
  Button,
  Checkbox,
  FormControlLabel,
  Grid,
  Paper,
} from '@mui/material';
import { CalculationParams } from '../types';

interface Props {
  params: CalculationParams;
  setParams: (params: CalculationParams) => void;
  onCalculate: () => void;
  loading: boolean;
}

const ParameterInputs: React.FC<Props> = React.memo(({
  params,
  setParams,
  onCalculate,
  loading
}) => {
  const handleChange = (field: keyof CalculationParams, value: any) => {
    setParams({ ...params, [field]: value });
  };

  const handleVector3Change = (
    field: 'rtxrx' | 'rsp' | 'mtx',
    component: 'x' | 'y' | 'z',
    value: number
  ) => {
    setParams({
      ...params,
      [field]: { ...params[field], [component]: value },
    });
  };

  // Update TextField common styles
  const commonTextFieldProps = {
    margin: "dense" as const,
    size: "small" as const,
    variant: "outlined" as const,
    InputLabelProps: {
      shrink: true,
      sx: {
        position: 'relative',
        transform: 'none',
        fontSize: '0.8rem',
        marginBottom: '8px',
        display: 'block',
        color: '#4b5563',
        top: '0',
        left: '0',
      }
    },
    sx: { 
      '& .MuiInputBase-input': {
        py: 0.75,
        fontSize: '0.8rem',
      },
      '& .MuiOutlinedInput-root': {
        backgroundColor: '#ffffff',
      },
      '& .MuiFormLabel-root': {
        position: 'relative',
        transform: 'none !important',
        top: '0 !important',
        left: '0 !important',
        marginBottom: '4px',
        '&.Mui-focused': {
          transform: 'none !important',
        }
      },
      '& .MuiInputLabel-shrink': {
        transform: 'none !important',
      },
      mb: 2,
      '& + .MuiTextField-root': {
        mt: 1,
      },
      '& legend': {
        display: 'none'
      },
      '& fieldset': {
        top: 0,
      }
    }
  };

  // Update section title styles
  const sectionTitleProps = {
    variant: "subtitle2" as const,
    sx: { 
      mb: 1,
      fontWeight: 600, 
      color: '#2c3e50',
      fontSize: '0.85rem',
    }
  };

  // Update Paper styles
  const paperProps = {
    elevation: 1,
    sx: { 
      p: 1.5,
      backgroundColor: '#fafafa',
      '&:not(:last-child)': {
        mb: 1
      }
    }
  };

  // Update vector label typography
  const VectorLabel = ({ children }: { children: React.ReactNode }) => (
    <Typography 
      variant="body2" 
      sx={{ 
        fontSize: '0.8rem',
        color: '#475569',
        mb: 1,
        mt: 1.5,
        fontWeight: 500,
      }}
    >
      {children}
    </Typography>
  );

  return (
    <Box sx={{ 
      display: 'flex',
      flexDirection: 'column',
      height: '100vh',
      maxHeight: '100%',
      position: 'relative',
    }}>
      {/* Fixed Header */}
      <Box sx={{ 
        py: 2,
        px: 2,
        borderBottom: '1px solid #e5e7eb',
        backgroundColor: '#fafafa',
      }}>
        <Typography variant="h6" sx={{ 
          color: '#6366f1', 
          fontWeight: 600,
          fontSize: '1.1rem',
        }}>
          Model Parameters
        </Typography>
      </Box>

      {/* Scrollable Content */}
      <Box sx={{ 
        flex: 1,
        overflowY: 'auto',
        overflowX: 'hidden',
        px: 0.5,
        py: 2,
        '&::-webkit-scrollbar': {
          width: '6px',
        },
        '&::-webkit-scrollbar-track': {
          background: '#f1f1f1',
          borderRadius: '3px',
        },
        '&::-webkit-scrollbar-thumb': {
          background: '#c7c7c7',
          borderRadius: '3px',
          '&:hover': {
            background: '#a6a6a6',
          },
        },
      }}>
        {/* Survey Config Section */}
        <Paper sx={{ 
          p: 2.5,
          mb: 2,
          backgroundColor: '#fafafa' 
        }}>
          <Typography sx={{ 
            fontSize: '0.9rem',
            fontWeight: 600,
            color: '#2c3e50',
            mb: 2.5
          }}>
            Survey Config
          </Typography>
          
          <TextField
            {...commonTextFieldProps}
            fullWidth
            label="Transmitter Height (m)"
            type="number"
            value={params.radar}
            onChange={(e) => handleChange('radar', Number(e.target.value))}
          />
          
          <VectorLabel>Tx-Rx Offset (m)</VectorLabel>
          <Grid container spacing={0.5}>
            <Grid item xs={4}>
              <TextField
                {...commonTextFieldProps}
                fullWidth
                label="X"
                type="number"
                value={params.rtxrx.x}
                onChange={(e) => handleVector3Change('rtxrx', 'x', Number(e.target.value))}
              />
            </Grid>
            <Grid item xs={4}>
              <TextField
                {...commonTextFieldProps}
                fullWidth
                label="Y"
                type="number"
                value={params.rtxrx.y}
                onChange={(e) => handleVector3Change('rtxrx', 'y', Number(e.target.value))}
              />
            </Grid>
            <Grid item xs={4}>
              <TextField
                {...commonTextFieldProps}
                fullWidth
                label="Z"
                type="number"
                value={params.rtxrx.z}
                onChange={(e) => handleVector3Change('rtxrx', 'z', Number(e.target.value))}
              />
            </Grid>
          </Grid>

          <TextField
            {...commonTextFieldProps}
            fullWidth
            label="Dipole Moment"
            type="number"
            value={params.dipole_m}
            onChange={(e) => handleChange('dipole_m', Number(e.target.value))}
          />

          <TextField
            {...commonTextFieldProps}
            fullWidth
            label="Pulse Length (s)"
            type="number"
            value={params.pulse_length}
            onChange={(e) => handleChange('pulse_length', Number(e.target.value))}
          />

          <TextField
            {...commonTextFieldProps}
            fullWidth
            label="Period (s)"
            type="number"
            value={params.period}
            onChange={(e) => handleChange('period', Number(e.target.value))}
          />

          <TextField
            {...commonTextFieldProps}
            fullWidth
            label="Profile Length (m)"
            type="number"
            value={params.profile_length}
            onChange={(e) => handleChange('profile_length', Number(e.target.value))}
          />
        </Paper>

        {/* Sphere & Overburden Section */}
        <Paper sx={{ 
          p: 2.5,
          mb: 2,
          backgroundColor: '#fafafa' 
        }}>
          <Typography sx={{ 
            fontSize: '0.9rem',
            fontWeight: 600,
            color: '#2c3e50',
            mb: 2.5
          }}>
            Sphere & Overburden
          </Typography>
          <TextField
            {...commonTextFieldProps}
            fullWidth
            label="Overburden Conductivity (S/m)"
            type="number"
            value={params.sigma_ob}
            onChange={(e) => handleChange('sigma_ob', Number(e.target.value))}
          />
          <TextField
            {...commonTextFieldProps}
            fullWidth
            label="Overburden Thickness (m)"
            type="number"
            value={params.thick_ob}
            onChange={(e) => handleChange('thick_ob', Number(e.target.value))}
          />
          <TextField
            {...commonTextFieldProps}
            fullWidth
            label="Sphere Conductivity (S/m)"
            type="number"
            value={params.sigma_sp}
            onChange={(e) => handleChange('sigma_sp', Number(e.target.value))}
          />
          <TextField
            {...commonTextFieldProps}
            fullWidth
            label="Sphere Radius (m)"
            type="number"
            value={params.a}
            onChange={(e) => handleChange('a', Number(e.target.value))}
          />
          
          <VectorLabel>Sphere Position (m)</VectorLabel>
          <Grid container spacing={0.5}>
            <Grid item xs={4}>
              <TextField
                {...commonTextFieldProps}
                fullWidth
                label="X"
                type="number"
                value={params.rsp.x}
                onChange={(e) => handleVector3Change('rsp', 'x', Number(e.target.value))}
              />
            </Grid>
            <Grid item xs={4}>
              <TextField
                {...commonTextFieldProps}
                fullWidth
                label="Y"
                type="number"
                value={params.rsp.y}
                onChange={(e) => handleVector3Change('rsp', 'y', Number(e.target.value))}
              />
            </Grid>
            <Grid item xs={4}>
              <TextField
                {...commonTextFieldProps}
                fullWidth
                label="Z"
                type="number"
                value={params.rsp.z}
                onChange={(e) => handleVector3Change('rsp', 'z', Number(e.target.value))}
              />
            </Grid>
          </Grid>
        </Paper>

        {/* Dipping Model Section */}
        <Paper sx={{ 
          p: 2.5,
          mb: 2,
          backgroundColor: '#fafafa' 
        }}>
          <Typography sx={{ 
            fontSize: '0.9rem',
            fontWeight: 600,
            color: '#2c3e50',
            mb: 2.5
          }}>
            Dipping Model
          </Typography>
          <TextField
            {...commonTextFieldProps}
            fullWidth
            label="Strike (degrees)"
            type="number"
            value={params.strike}
            onChange={(e) => handleChange('strike', Number(e.target.value))}
          />
          <TextField
            {...commonTextFieldProps}
            fullWidth
            label="Dip (degrees)"
            type="number"
            value={params.dip}
            onChange={(e) => handleChange('dip', Number(e.target.value))}
          />
          <FormControlLabel
            control={
              <Checkbox
                checked={params.apply_dip}
                onChange={(e) => handleChange('apply_dip', e.target.checked)}
                size="small"
                sx={{ 
                  padding: '2px',
                  '& .MuiSvgIcon-root': { fontSize: 18 }
                }}
              />
            }
            label={<Typography sx={{ fontSize: '0.8rem' }}>Apply Dip</Typography>}
            sx={{ marginLeft: -1 }}  // Reduce left margin
          />
        </Paper>
      </Box>

      {/* Fixed Footer with Button */}
      <Box sx={{ 
        p: 2,
        borderTop: '1px solid #e5e7eb',
        backgroundColor: '#ffffff',
      }}>
        <Button
          variant="contained"
          color="primary"
          onClick={onCalculate}
          disabled={loading}
          fullWidth
          sx={{
            py: 1.5,
            textTransform: 'none',
            fontSize: '1rem',
            fontWeight: 600,
            background: 'linear-gradient(45deg, #6366f1 0%, #818cf8 100%)',
            '&:hover': {
              background: 'linear-gradient(45deg, #4f46e5 0%, #6366f1 100%)',
            },
          }}
        >
          {loading ? 'Calculating...' : 'Calculate Response'}
        </Button>
      </Box>
    </Box>
  );
});

export default ParameterInputs; 