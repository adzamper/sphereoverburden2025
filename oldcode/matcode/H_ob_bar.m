function [ H_barx,H_barz ] = H_ob_bar( a,T,sigma_sp,mtx,dipoleM,rtx,rsp,rtxrx,t,Tau,mu,sigma_ob,thick_ob )
% This function calculates the innermost integral involved for calculating
% the 2nd order term of the induced moment at the sphere (equation 19 of 
% Desmarais and Smith, 2016. Geophysics 81(4), P. E265-E277)
rsp_x=0;
rsp_y=0;
rsp_z=rsp(3);
rrx_x=rtx(1)-rtxrx(1)-rsp(1);
rrx_y=rtx(2)-rtxrx(2)-rsp(2);
rrx_z=rtx(3)-rtxrx(3);
if length(Tau) > 1
    convo_x=@(tau) 2.*pi.*a^3.*(-dH_obdt_x( mtx,dipoleM,[0,0,rtx(3)],[rsp(1)-rtx(1),rsp(2)-rtx(2),rsp(3)],tau,mu,sigma_ob,thick_ob ).*Thetafunction( Tau,tau,0,mu,sigma_sp,a,T )-dH_obdt_x( mtx,dipoleM,[0,0,rtx(3)],[rsp(1)-rtx(1),rsp(2)-rtx(2),rsp(3)],Tau,mu,sigma_ob,thick_ob )-(H_ob_x( mtx,dipoleM,[0,0,rtx(3)],[rsp(1)-rtx(1),rsp(2)-rtx(2),rsp(3)],0,mu,sigma_ob,thick_ob ).*Thetafunction( Tau,0,0,mu,sigma_sp,a,T )));
    convo_z=@(tau) 2.*pi.*a^3.*(-dH_obdt_z( mtx,dipoleM,[0,0,rtx(3)],[rsp(1)-rtx(1),rsp(2)-rtx(2),rsp(3)],tau,mu,sigma_ob,thick_ob ).*Thetafunction( Tau,tau,0,mu,sigma_sp,a,T )-dH_obdt_z( mtx,dipoleM,[0,0,rtx(3)],[rsp(1)-rtx(1),rsp(2)-rtx(2),rsp(3)],Tau,mu,sigma_ob,thick_ob )-(H_ob_z( mtx,dipoleM,[0,0,rtx(3)],[rsp(1)-rtx(1),rsp(2)-rtx(2),rsp(3)],0,mu,sigma_ob,thick_ob ).*Thetafunction( Tau,0,0,mu,sigma_sp,a,T )));

    taumat=zeros(length(Tau),length(Tau));
    for j=1:length(Tau)
        for i=1:length(Tau)-1
            taumat(j,(i+1))=taumat(j,i)+(t-Tau(j));
        end
    end

    for i=1:length(Tau)
        tau=taumat(i,:);
        msp_x=convo_x(tau);
        msp_z=convo_z(tau);
        fun_x= (1/(4*pi)).*((-3.*((rrx_x - rsp_x).^2 + (rrx_y - rsp_y).^2 + (rrx_z - rsp_z - (2.*(t-Tau))./(mu.*sigma_ob.*thick_ob)).^2).^(-5./2).*(rrx_x - rsp_x).*((msp_x.*(rrx_x - rsp_x))+(0.*(rrx_y - rsp_y))+(2.*pi.*a^3.*msp_z.*(rrx_z - rsp_z - (2.*(t-Tau))./(mu.*sigma_ob.*thick_ob)))))+(((rrx_x - rsp_x).^2 + (rrx_y - rsp_y).^2 + (rrx_z - rsp_z - (2.*(t-Tau))./(mu.*sigma_ob.*thick_ob)).^2).^(-3./2).*msp_x));
        fun_z= (1/(4*pi)).*((-3.*((rrx_x - rsp_x).^2 + (rrx_y - rsp_y).^2 + (rrx_z - rsp_z - (2.*(t-Tau))./(mu.*sigma_ob.*thick_ob)).^2).^(-5./2).*(rrx_z - rsp_z - (2.*(t-Tau))./(mu.*sigma_ob.*thick_ob)).*((msp_x.*(rrx_x - rsp_x))+(0.*(rrx_y - rsp_y))+(2.*pi.*a^3.*msp_z.*(rrx_z - rsp_z - (2.*(t-Tau))./(mu.*sigma_ob.*thick_ob)))))+(((rrx_x - rsp_x).^2 + (rrx_y - rsp_y).^2 + (rrx_z - rsp_z - (2.*(t-Tau))./(mu.*sigma_ob.*thick_ob)).^2).^(-3./2).*2.*pi.*a^3.*msp_z));
        size(tau)
        size(fun_x)
        H_barx=trapz(tau,fun_x);
        H_barz=trapz(tau,fun_z);
    end
else
    fun_x=@(tau) (1/(4*pi)).*((-3.*((rrx_x - rsp_x).^2 + (rrx_y - rsp_y).^2 + (rrx_z - rsp_z - (2.*(t-Tau))./(mu.*sigma_ob.*thick_ob)).^2).^(-5./2).*(rrx_x - rsp_x).*((2.*pi.*a^3.*(-dH_obdt_x( mtx,dipoleM,[0,0,rtx(3)],[rsp(1)-rtx(1),rsp(2)-rtx(2),rsp(3)],tau,mu,sigma_ob,thick_ob ).*Thetafunction( Tau,tau,0,mu,sigma_sp,a,T )-dH_obdt_x( mtx,dipoleM,[0,0,rtx(3)],[rsp(1)-rtx(1),rsp(2)-rtx(2),rsp(3)],Tau,mu,sigma_ob,thick_ob )).*(rrx_x - rsp_x))+(0.*(rrx_y - rsp_y))+(2.*pi.*a^3.*2.*pi.*a^3.*(-dH_obdt_z( mtx,dipoleM,[0,0,rtx(3)],[rsp(1)-rtx(1),rsp(2)-rtx(2),rsp(3)],tau,mu,sigma_ob,thick_ob ).*Thetafunction( Tau,tau,0,mu,sigma_sp,a,T )-dH_obdt_z( mtx,dipoleM,[0,0,rtx(3)],[rsp(1)-rtx(1),rsp(2)-rtx(2),rsp(3)],Tau,mu,sigma_ob,thick_ob )).*(rrx_z - rsp_z - (2.*(t-Tau))./(mu.*sigma_ob.*thick_ob)))))+(((rrx_x - rsp_x).^2 + (rrx_y - rsp_y).^2 + (rrx_z - rsp_z - (2.*(t-Tau))./(mu.*sigma_ob.*thick_ob)).^2).^(-3./2).*2.*pi.*a^3.*(-dH_obdt_x( mtx,dipoleM,[0,0,rtx(3)],[rsp(1)-rtx(1),rsp(2)-rtx(2),rsp(3)],tau,mu,sigma_ob,thick_ob ).*Thetafunction( Tau,tau,0,mu,sigma_sp,a,T )-dH_obdt_x( mtx,dipoleM,[0,0,rtx(3)],[rsp(1)-rtx(1),rsp(2)-rtx(2),rsp(3)],Tau,mu,sigma_ob,thick_ob ))));
    fun_z=@(tau) (1/(4*pi)).*((-3.*((rrx_x - rsp_x).^2 + (rrx_y - rsp_y).^2 + (rrx_z - rsp_z - (2.*(t-Tau))./(mu.*sigma_ob.*thick_ob)).^2).^(-5./2).*(rrx_z - rsp_z - (2.*(t-Tau))./(mu.*sigma_ob.*thick_ob)).*((2.*pi.*a^3.*(-dH_obdt_x( mtx,dipoleM,[0,0,rtx(3)],[rsp(1)-rtx(1),rsp(2)-rtx(2),rsp(3)],tau,mu,sigma_ob,thick_ob ).*Thetafunction( Tau,tau,0,mu,sigma_sp,a,T )-dH_obdt_x( mtx,dipoleM,[0,0,rtx(3)],[rsp(1)-rtx(1),rsp(2)-rtx(2),rsp(3)],Tau,mu,sigma_ob,thick_ob )).*(rrx_x - rsp_x))+(0.*(rrx_y - rsp_y))+(2.*pi.*a^3.*2.*pi.*a^3.*(-dH_obdt_z( mtx,dipoleM,[0,0,rtx(3)],[rsp(1)-rtx(1),rsp(2)-rtx(2),rsp(3)],tau,mu,sigma_ob,thick_ob ).*Thetafunction( Tau,tau,0,mu,sigma_sp,a,T )-dH_obdt_z( mtx,dipoleM,[0,0,rtx(3)],[rsp(1)-rtx(1),rsp(2)-rtx(2),rsp(3)],Tau,mu,sigma_ob,thick_ob )).*(rrx_z - rsp_z - (2.*(t-Tau))./(mu.*sigma_ob.*thick_ob)))))+(((rrx_x - rsp_x).^2 + (rrx_y - rsp_y).^2 + (rrx_z - rsp_z - (2.*(t-Tau))./(mu.*sigma_ob.*thick_ob)).^2).^(-3./2).*2.*pi.*a^3.*2.*pi.*a^3.*(-dH_obdt_z( mtx,dipoleM,[0,0,rtx(3)],[rsp(1)-rtx(1),rsp(2)-rtx(2),rsp(3)],tau,mu,sigma_ob,thick_ob ).*Thetafunction( Tau,tau,0,mu,sigma_sp,a,T )-dH_obdt_z( mtx,dipoleM,[0,0,rtx(3)],[rsp(1)-rtx(1),rsp(2)-rtx(2),rsp(3)],Tau,mu,sigma_ob,thick_ob ))));
    H_barx=integral(fun_x,0,t-Tau);
    H_barz=integral(fun_z,0,t-Tau);
end