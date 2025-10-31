function [ H_ob_x ] = dH_obdt_x( mtx,dipoleM,rtx,rrx,O,mu,sigma_ob,thick_ob )
% This function evaluates the time-derivative of the x component 
% of the field of the  overburden alone (equation A-5a of 
% Desmarais and Smith, 2016. Geophysics 81(4), P. E265-E277)
m_x=dipoleM.*mtx(1);
m_y=dipoleM.*mtx(2);
m_z=dipoleM.*mtx(3);
rtx_x=rtx(1);
rtx_y=rtx(2);
rtx_z=rtx(3);
rrx_x=rrx(1);
rrx_y=rrx(2);
rrx_z=rrx(3);

if rrx_z > 0    
H_ob_x=(-1./(4.*pi)).*((m_z.*(6.*rrx_x - 6.*rtx_x))./(mu.*sigma_ob.*thick_ob.*((rrx_x - rtx_x).^2 + (rrx_y - rtx_y).^2 + (rrx_z + rtx_z + (2.*O)./(mu.*sigma_ob.*thick_ob)).^2).^(5./2)) - (6.*m_x.*(rrx_z + rtx_z + (2.*O)./(mu.*sigma_ob.*thick_ob)))./(mu.*sigma_ob.*thick_ob.*((rrx_x - rtx_x).^2 + (rrx_y - rtx_y).^2 + (rrx_z + rtx_z + (2.*O)./(mu.*sigma_ob.*thick_ob)).^2).^(5./2)) + (5.*(6.*rrx_x - 6.*rtx_x).*(rrx_z + rtx_z + (2.*O)./(mu.*sigma_ob.*thick_ob)).*(m_x.*(rrx_x - rtx_x) - m_z.*(rrx_z + rtx_z + (2.*O)./(mu.*sigma_ob.*thick_ob)) + m_y.*(rrx_y - rtx_y)))./(mu.*sigma_ob.*thick_ob.*((rrx_x - rtx_x).^2 + (rrx_y - rtx_y).^2 + (rrx_z + rtx_z + (2.*O)./(mu.*sigma_ob.*thick_ob)).^2).^(7./2)));
else
H_ob_x=(-1./(4.*pi)).*((m_z.*(6.*rrx_x - 6.*rtx_x))./(mu.*sigma_ob.*thick_ob.*((rrx_x - rtx_x).^2 + (rrx_y - rtx_y).^2 + (rtx_z - rrx_z + (2.*O)./(mu.*sigma_ob.*thick_ob)).^2).^(5./2)) - (6.*m_x.*(rtx_z - rrx_z + (2.*O)./(mu.*sigma_ob.*thick_ob)))./(mu.*sigma_ob.*thick_ob.*((rrx_x - rtx_x).^2 + (rrx_y - rtx_y).^2 + (rtx_z - rrx_z + (2.*O)./(mu.*sigma_ob.*thick_ob)).^2).^(5./2)) + (5.*(6.*rrx_x - 6.*rtx_x).*(rtx_z - rrx_z + (2.*O)./(mu.*sigma_ob.*thick_ob)).*(m_x.*(rrx_x - rtx_x) + m_y.*(rrx_y - rtx_y) - m_z.*(rtx_z - rrx_z + (2.*O)./(mu.*sigma_ob.*thick_ob))))./(mu.*sigma_ob.*thick_ob.*((rrx_x - rtx_x).^2 + (rrx_y - rtx_y).^2 + (rtx_z - rrx_z + (2.*O)./(mu.*sigma_ob.*thick_ob)).^2).^(7./2)));
end
