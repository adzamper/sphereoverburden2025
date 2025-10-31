function [ H_tot_x,H_tot_y,H_tot_z,H_x,H_y,H_z,convo_x,convo_z ] = H_total_step_1storder( mtx,dipoleM,rtx,rtxrx,rsp,t,mu,sigma_ob,thick_ob,sigma_sp,a,applydip,dip,strike,T )
%this function calculates the sphere-overburden response in second order


temp=2*pi*a^3*dH_tot_x_step( mtx,dipoleM,[0,0,rtx(3)],[-rtx(1),-rtx(2),rsp(3)],mu,sigma_ob,thick_ob,t,0,sigma_sp,a,T );
%store x component of induced moment
convo_x=temp;
convo_y=0;
H_tot_y=0;
H_y=0;
%This loop convolves with the tx waveform for z component
temp=2*pi*a^3*dH_tot_z_step( mtx,dipoleM,[0,0,rtx(3)],[-rtx(1),-rtx(2),rsp(3)],mu,sigma_ob,thick_ob,t,0,sigma_sp,a,T );
%store z component of induced moment
convo_z=temp;

%store sphere moment
msp=[convo_x,convo_y,convo_z];

%dipping sphere model if applydip=1
if (applydip==1) 
  norm=[cosd(90.-dip)*cosd(strike-90.),sind(strike-90.)*cosd(90.-dip),sind(90.-dip)];
% make the dip normal vector a unit vector
  normt=sqrt(dot (norm,norm));
  norm=norm/normt;
end
if (applydip==1) 
  mspdotnorm=dot(msp,norm);
% now scale the normal to have this strength and redirect the sphere 
% moment to be in the dip direction
  msp=mspdotnorm*norm;
end

%calculate field using induced moment
H_tot_x=-dot([1,0,0],static(msp,[-rtxrx(1),-rtxrx(2),rtx(3)-rtxrx(3)]-[-rtx(1),-rtx(2),rsp(3)]));
H_tot_z=dot([0,0,1],static(msp,[-rtxrx(1),-rtxrx(2),rtx(3)-rtxrx(3)]-[-rtx(1),-rtx(2),rsp(3)]));

%calculate 0th order term (field of overburden alone) and convolve with
%waveform
H_x=H_ob_x( mtx,dipoleM,[0,0,rtx(3)],[-rtxrx(1),-rtxrx(2),rtx(3)-rtxrx(3)],t,mu,sigma_ob,thick_ob );
H_tot_x=H_tot_x+H_x;

H_z=H_ob_z( mtx,dipoleM,[0,0,rtx(3)],[-rtxrx(1),-rtxrx(2),rtx(3)-rtxrx(3)],t,mu,sigma_ob,thick_ob );
H_tot_z=H_tot_z+H_z;