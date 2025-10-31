function [ H_x,H_z ] = H_2ndorder( a,T,sigma_sp,mtx,dipoleM,rtx,rsp,rtxrx,t,mu,sigma_ob,thick_ob )
%This function calculates the outermost integral of equation (19) in 
% Desmarais and Smith, 2016. Geophysics 81(4), P. E265-E277
fun=@(Tau) H_ob_bar( a,T,sigma_sp,mtx,dipoleM,rtx,rsp,rtxrx,t,Tau,mu,sigma_ob,thick_ob );
fun1=fun(1);
fun2=fun(2);
fun_x=@(Tau) fun1;
fun_z=@(Tau) fun2;
H_x=integral(fun_x,0,t,'Arrayvalued',true);
H_z=integral(fun_z,0,t,'Arrayvalued',true);
end
