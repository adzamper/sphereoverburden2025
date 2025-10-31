function [ int ] = dH_tot_x_step( mtx,dipoleM,rtx,rsp,mu,sigma_ob,thick_ob,t,o,sigma_sp,a,T )
%This function calculates the x component of the first order induced moment at the sphere
%using equation (16b) of Desmarais and Smith, 2016. Geophysics 81(4), P. E265-E277
    fun=@(O) -dH_obdt_x( mtx,dipoleM,rtx,rsp,O,mu,sigma_ob,thick_ob ).*Thetafunction_step( t,O,o,mu,sigma_sp,a,T );
    int=integral(fun,0,t-o,'RelTol',1e-5,'AbsTol',1e-20)+(H_ob_x( mtx,dipoleM,rtx,rsp,-o,mu,sigma_ob,thick_ob ).*Thetafunction_step( t,0,o,mu,sigma_sp,a,T ));