function [ h ] = static( m,r )
%static calculates the field of a dipole
%   m is the magnetic field vector
%   r is the vector from the dipole to the field location
%   m is the dipole moment vector
% Static magnetic field (hx,hy,hz) at r=(x,y,z)   [ or:  Bx,By,Bz ]
% of a magnetic dipole (mx,my,mz) at (0,0,0)
%
%               1        3 m.r
%   h   =   -------- (  ------- r   -   m  )
%   -       4*pi*r^3      r^2   -       -
%
% The units of h are the same as those of m. So if you want the B-field
% then multiply all components of mm by mu0 before calling this routine !
one_over_4pi=1./(4.*3.14159265358979);
r2=dot(r,r);
if (r2<1.e-20)
    h=0.0;
else
    a=(one_over_4pi)/(sqrt(r2)*r2);
    b=dot(r,m)*3./r2;
    h=(b*r-m)*a;
end
end
