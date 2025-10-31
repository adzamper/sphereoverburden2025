%--------------------------------------------------------------------------
%This is the main script to evaluate the electromagnetic response of a
%dipping sphere under a thin sheet of conductive overburden. The x
%component of the response has an inverted sign because of the choice of
%the axis system which is opposite to that of the MEGATEM tx-rx geometry.
%The y component has not yet been coded, because we only did the 0 offset case
%(but this shouldnt be difficult and I can help you do this if you'd like)
%All units are in SI
%Cite: Desmarais and Smith, 2016. Geophysics 81(4), P. E265-E277
%--------------------------------------------------------------------------
rtx(1)=0.;%tx coordinate vector
rtx(2)=0.;
radar=120.;%height of transmitter above ground surface
rtx(3)=radar;
mu=1.256637e-6; %permeability of free space
dipoleM=1.847300*1e6; %dipole moment of tx
rtxrx(1)=125.;  %vector offset from tx to rx
rtxrx(2)=0.;
rtxrx(3)=56.;
rrx(1)=rtx(1)-rtxrx(1);% position of rx
rrx(2)=rtx(2)-rtxrx(2);
rrx(3)=rtx(3)-rtxrx(3);
rsp(1)=0.;%sphere position
rsp(2)=0.;
rsp(3)=-200.;
a=100.0;%sphere radius
sigma_sp=0.5;%sphere conductivity
mtx(1)=0.;%unit vector of tx dipole moment
mtx(2)=0.;
mtx(3)=1.;
interval=101;%number of times field is calculated along profile
profile=zeros(1,interval);%profile position vector
profile_rrx=zeros(1,interval);
%window centers
wc=[0.000154600000000000;0.000236000000000000;0.000333700000000000;0.000447600000000000;0.000577800000000000;0.000740600000000000;0.000944000000000000;0.00118820000000000;0.00151370000000000;0.00192060000000000;0.00253090000000000;0.00334470000000000;0.00456540000000000;0.00619300000000000;0.00901430000000000];
nw=length(wc);      %number of windows
P=3.65*1E-3;   %pulse length
bfreq=30; %frequency of transmitter waveform
T=1/bfreq; %period
H_tot_x=zeros(nw,interval); %response vectors
H_tot_y=zeros(nw,interval);
H_tot_z=zeros(nw,interval);
C_x=zeros(nw,interval); %induced sphere moment vectors
C_z=zeros(nw,interval);
H_ob1=zeros(nw,interval); % overburden respose vectors
H_ob2=zeros(nw,interval);
H_ob3=zeros(nw,interval);
sigma_ob=1/30;%conductivity of overburden in S/m
thick_ob=4;%thickness of overburden in m
delta_x=1600/((interval-1)); %length of interval along profile
applydip=0; %if 1 then apply dipping sphere model
strike=90.;%strike of sphere
dip=85.;%dip of sphere

%move transmitter along profile and calculate response of overburden
for j=1:nw %iterate time
    i=0;
    j
    tic;
    for x=-800:delta_x:800 %iterate along profile
        i=i+1;
        i
        profile(i)=x-rtxrx(1);
        rtx(1)=x;
        rrx(1)=rtx(1)-rtxrx(1);
        %calculate response
        [H_x,H_y,H_z,H_obx,H_oby,H_obz,convo_x,convo_z]=H_total_step_1storder( mtx,dipoleM,rtx,rtxrx,rsp,wc(j),mu,sigma_ob,thick_ob,sigma_sp,a,applydip,dip,strike,T );
        H_tot_x(j,i)=(mu/1e-12).*H_x;
        H_tot_y(j,i)=(mu/1e-12).*H_y;
        H_tot_z(j,i)=(mu/1e-12).*H_z;
        C_x(j,i)=convo_x;
        C_z(j,i)=convo_z;
        H_ob1(j,i)=H_obx;
        H_ob2(j,i)=H_oby;
        H_ob3(j,i)=H_obz;
    end
    toc
end
%plot response
figure(1)
plot(profile,H_tot_x)
title('x component','FontSize',14,'FontWeight','bold')
ylabel(' Response','FontSize',14,'FontWeight','bold')
xlabel('Profile position (m)','FontSize',14,'FontWeight','bold')
set(gca,'FontSize',14,'FontWeight','bold')
ylim([min(min(H_tot_x)) max(max(H_tot_x))])
figure(2)
plot(profile,H_tot_y)
title('y component','FontSize',14,'FontWeight','bold')
ylabel(' Response (A/m)','FontSize',14,'FontWeight','bold')
xlabel('Profile position (m)','FontSize',14,'FontWeight','bold')
set(gca,'FontSize',14,'FontWeight','bold')
if max(max(H_tot_y)) > 0
ylim([min(min(H_tot_y)) max(max(H_tot_y))])
end
figure(3)
plot(profile,H_tot_z)
title('z component','FontSize',14,'FontWeight','bold')
ylabel('Response (A/m)','FontSize',14,'FontWeight','bold')
xlabel('Profile position (m)','FontSize',14,'FontWeight','bold')
set(gca,'FontSize',14,'FontWeight','bold')
ylim([min(min(H_tot_z)) max(max(H_tot_z))])