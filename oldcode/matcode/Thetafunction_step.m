function [ theta ] = Thetafunction_step( t,O,o,mu,sigma_sp,a,T)
% This function calculates the time-dependant part of the step-response of
% the sphere alone (equations 12-13 in Desmarais and Smith, 2016.
% Geophysics 81(4), P. E265-E277)
% The function also includes the pre-factor in equation 21b of 
% Desmarais and Smith, 2016. Geophysics 81(4), P. E265-E277 
ss=mu*sigma_sp*a*a;
temp=inf;
theta=0;
k=0;
Ton2=T/2;
while (theta/temp) < 1E6
    k=k+1;
    temp=(1/(1+exp(-Ton2.*((k.*pi)^2)./ss))).*((6./((k.*pi)^2)).*exp((o+O-t).*((k.*pi)^2)./ss));
    theta=theta+temp;
end