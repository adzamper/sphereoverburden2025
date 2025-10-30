import React, { useState } from 'react';
import Plot from 'react-plotly.js';
import { Box, Typography, Paper, FormGroup, FormControlLabel, Checkbox } from '@mui/material';
import { ResponseData } from '../types';
import { Data } from 'plotly.js';

interface Props {
  response: ResponseData | null;
}

const ResponsePlots: React.FC<Props> = ({ response }) => {
  const [visiblePlots, setVisiblePlots] = useState({
    x: true,
    y: false,
    z: true
  });

  if (!response) {
    return (
      <Box sx={{ p: 3, textAlign: 'center' }}>
        <Typography>No data to display. Please calculate a response.</Typography>
      </Box>
    );
  }

  const commonLayout = {
    showlegend: false,
    xaxis: {
      title: 'Profile Position (m)',
      zeroline: true,
      zerolinecolor: '#94a3b8',
      gridcolor: '#e2e8f0',
      zerolinewidth: 1.5,
      tickfont: { size: 12 },
    },
    margin: {
      l: 60,
      r: 20,
      b: 40,
      t: 40,
      pad: 4
    },
    height: 260,
    plot_bgcolor: '#ffffff',
    paper_bgcolor: '#ffffff',
    hoverlabel: {
      bgcolor: '#ffffff',
      font: { family: 'Inter, sans-serif', size: 14 },
      bordercolor: '#6366f1',
    },
    font: {
      family: 'Inter, sans-serif',
    },
    yaxis: {
      gridcolor: '#e2e8f0',
      zerolinecolor: '#94a3b8',
      zerolinewidth: 1.5,
      tickfont: { size: 12 },
    },
  };

  const createTraces = (components: number[][]): Data[] => {
    // Create a gradient of colors that are easily distinguishable
    const generateColors = (count: number) => {
      return Array.from({ length: count }, (_, i) => {
        const hue = 230 + (i * 15); // Start from indigo/blue and vary the hue
        const saturation = 85 - (i * 3); // Slightly decrease saturation
        const lightness = 65 - (i * 2); // Slightly decrease lightness
        return `hsl(${hue}, ${saturation}%, ${lightness}%)`;
      });
    };

    const colors = generateColors(response.time_windows.length);

    return response.time_windows.map((time, i) => ({
      x: response.x_values,
      y: components[i],
      type: 'scatter' as const,
      mode: 'lines' as const,
      line: {
        color: colors[i],
        width: 2,
      },
      hoverinfo: 'y+x+name' as const,
      name: `Window ${i + 1}`,
      hoverlabel: {
        namelength: -1
      },
    }));
  };

  const activePlotCount = Object.values(visiblePlots).filter(Boolean).length;
  const plotHeight = activePlotCount > 0 ? Math.floor(780 / activePlotCount) : 260; // Dynamically adjust height

  const updatedLayout = {
    ...commonLayout,
    height: plotHeight,
  };

  return (
    <Box sx={{ 
      backgroundColor: '#ffffff', 
      borderRadius: 2,
      p: 2,
      height: '100%',
      display: 'flex',
      flexDirection: 'column',
    }}>
      <Box sx={{ 
        display: 'flex', 
        justifyContent: 'space-between', 
        alignItems: 'center',
        mb: 2,
        borderBottom: '2px solid #e3f2fd',
        pb: 1
      }}>
        <Typography 
          variant="h6" 
          sx={{ 
            color: '#6366f1', 
            fontWeight: 600,
          }}
        >
          Response Components
        </Typography>
        <FormGroup row sx={{ gap: 2 }}>
          <FormControlLabel
            control={
              <Checkbox
                checked={visiblePlots.x}
                onChange={(e) => setVisiblePlots(prev => ({ ...prev, x: e.target.checked }))}
                size="small"
              />
            }
            label="X Component"
          />
          <FormControlLabel
            control={
              <Checkbox
                checked={visiblePlots.y}
                onChange={(e) => setVisiblePlots(prev => ({ ...prev, y: e.target.checked }))}
                size="small"
              />
            }
            label="Y Component"
          />
          <FormControlLabel
            control={
              <Checkbox
                checked={visiblePlots.z}
                onChange={(e) => setVisiblePlots(prev => ({ ...prev, z: e.target.checked }))}
                size="small"
              />
            }
            label="Z Component"
          />
        </FormGroup>
      </Box>
      
      <Box sx={{ 
        display: 'flex', 
        flexDirection: 'column', 
        gap: 2,
        flex: 1,
      }}>
        {visiblePlots.x && (
          <Paper elevation={2} sx={{ p: 2, borderRadius: 3 }}>
            <Plot
              data={createTraces(response.x_components)}
              layout={{
                ...updatedLayout,
                yaxis: { 
                  title: 'X Component (nT)',
                  gridcolor: '#e2e8f0',
                  zerolinecolor: '#94a3b8',
                  zerolinewidth: 1.5,
                },
                title: {
                  text: 'X Component Response',
                  font: { size: 16, color: '#2c3e50', weight: 600 }
                }
              }}
              config={{ responsive: true, displayModeBar: false }}
              style={{ width: '100%' }}
            />
          </Paper>
        )}

        {visiblePlots.z && (
          <Paper elevation={2} sx={{ p: 2, borderRadius: 3 }}>
            <Plot
              data={createTraces(response.z_components)}
              layout={{
                ...updatedLayout,
                yaxis: { 
                  title: 'Z Component (nT)',
                  gridcolor: '#e2e8f0',
                  zerolinecolor: '#94a3b8',
                  zerolinewidth: 1.5,
                },
                title: {
                  text: 'Z Component Response',
                  font: { size: 16, color: '#2c3e50', weight: 600 }
                }
              }}
              config={{ responsive: true, displayModeBar: false }}
              style={{ width: '100%' }}
            />
          </Paper>
        )}

        {visiblePlots.y && (
          <Paper elevation={2} sx={{ p: 2, borderRadius: 3 }}>
            <Plot
              data={createTraces(response.y_components)}
              layout={{
                ...updatedLayout,
                yaxis: { 
                  title: 'Y Component (nT)',
                  gridcolor: '#e2e8f0',
                  zerolinecolor: '#94a3b8',
                  zerolinewidth: 1.5,
                },
                title: {
                  text: 'Y Component Response',
                  font: { size: 16, color: '#2c3e50', weight: 600 }
                }
              }}
              config={{ responsive: true, displayModeBar: false }}
              style={{ width: '100%' }}
            />
          </Paper>
        )}
      </Box>
    </Box>
  );
};

export default ResponsePlots; 