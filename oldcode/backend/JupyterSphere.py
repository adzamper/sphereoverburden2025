from __future__ import print_function
from __future__ import absolute_import
from __future__ import unicode_literals

import numpy as np
import matplotlib.pyplot as plt
from scipy.constants import mu_0, epsilon_0

from ipywidgets import interact, interactive, IntSlider, widget, FloatText, FloatSlider
import matplotlib
import math
import warnings
warnings.filterwarnings('ignore')

def sphereresponse(aw, sigob, sigsp, thickob, depth, txheight, dipw):
    # Overburden Field Variables:

    mu = 1.256637e-6  # permeability of free space
    #print(mu)
    dipole_m = 1847300  # dipole moment of tx

    rtx = np.array([0, 0, txheight], dtype=np.int64)  # Tx co-ordinates in array
    radar = rtx[2]  # height of transmitter above ground surface

    offset_tx_rx = np.array([125, 0, 56], dtype=np.int64)  # offset from transmitter to receiver array

    rrx = np.array([rtx[0] - offset_tx_rx[0], rtx[1] - offset_tx_rx[1],
                    rtx[2] - offset_tx_rx[2]], dtype=np.int64)  # receiver co-ordinate array

    # sphere position array
    rsp = np.array([0, 0, depth], dtype=np.int64)
    a = aw  # sphere radius
    sigma_sp = sigsp  # sphere conductivity
    #sigma_sp = 0.5  # sphere conductivity

    mtx = np.array([0, 0, 1], dtype=np.int64)  # unit vector of transmitter dipole moment

    interval = 101  # number of times field is calculated along profile

    profile = np.zeros((1, interval))  # profile position vector
    profile_rrx = np.zeros((1, interval))

    # window centers

    wc = np.array([0.000154600000000000, 0.000236000000000000, 0.000333700000000000, 0.000447600000000000,
                   0.000577800000000000, 0.000740600000000000, 0.000944000000000000, 0.00118820000000000,
                   0.00151370000000000, 0.00192060000000000, 0.00253090000000000, 0.00334470000000000,
                   0.00456540000000000, 0.00619300000000000, 0.00901430000000000])

    # wave = df = pd.ExcelFile('test_megatem.xlsx').parse('PTA_MEGATEM_30Hz_20ch') #you could add index_col=0 if there's an index
    # x=[]
    # x.append(df['current_pulse'])
    # print(x)

    nw = len(wc)  # number of windows

    P = 3.65 * 1E-3  # pulse length

    bfreq = 30  # frequency of transmitter waveform

    T = 1 / bfreq  # period

    H_tot_x = np.zeros((nw, interval))  # response vectors
    H_tot_y = np.zeros((nw, interval))
    H_tot_z = np.zeros((nw, interval))

    C_x = np.zeros((nw, interval))  # induced sphere moment vectors
    C_z = np.zeros((nw, interval))

    H_ob1 = np.zeros((nw, interval))  # overburden response vectors
    H_ob2 = np.zeros((nw, interval))
    H_ob3 = np.zeros((nw, interval))

    #sigma_ob = 1 / 30  # conductivity of overburden in S/m
    sigma_ob = sigob
    #thick_ob = 2  # thickness of overburden in m
    thick_ob = thickob
    delta_x = (1600 / (interval - 1))  # length of interval along profile
    apply_dip = 1  # if 1 then apply dipping sphere model
    strike = 90  # strike of sphere
    dip = dipw # dip of sphere

    def dh_obdt_xyz(mtx, dipole_m, rtx, rrx, O, mu, sigma_ob, thick_ob):
        m_x = dipole_m * mtx[0]
        m_y = dipole_m * mtx[1]
        m_z = dipole_m * mtx[2]
        rtx_x = rtx[0]
        rtx_y = rtx[1]
        rtx_z = rtx[2]
        rrx_x = rrx[0]
        rrx_y = rrx[1]
        rrx_z = rrx[2]

        if rrx_z > 0:
            dh_obx = (-1 / (4 * math.pi)) * ((m_z * (6 * rrx_x - 6 * rtx_x)) / (mu * sigma_ob * thick_ob * (
                    (rrx_x - rtx_x) ** 2 + (rrx_y - rtx_y) ** 2 + (rrx_z + rtx_z + (2 * O) / (
                    mu * sigma_ob * thick_ob)) ** 2) ** (5 / 2)) - (6 * m_x * (rrx_z + rtx_z + (2 * O) / (
                    mu * sigma_ob * thick_ob))) / (mu * sigma_ob * thick_ob * ((rrx_x - rtx_x) ** 2 + (
                    rrx_y - rtx_y) ** 2 + (rrx_z + rtx_z + (2 * O) / (
                    mu * sigma_ob * thick_ob)) ** 2) ** (5 / 2)) + (5 * (6 * rrx_x - 6 * rtx_x) * (
                    rrx_z + rtx_z + (2 * O) / (mu * sigma_ob * thick_ob)) * (
                                                                            m_x * (rrx_x - rtx_x) - m_z * (
                                                                                rrx_z + rtx_z + (2 * O) / (
                                                                                mu * sigma_ob * thick_ob)) + m_y * (
                                                                                        rrx_y - rtx_y))) / (
                                                     mu * sigma_ob * thick_ob * ((rrx_x - rtx_x) ** 2 + (
                                                     rrx_y - rtx_y) ** 2 + (rrx_z + rtx_z + (2 * O) / (
                                                     mu * sigma_ob * thick_ob)) ** 2) ** (7 / 2)))

        else:
            # (-1 / (4 * math.pi))
            dh_obx1 = ((m_z * (6 * rrx_x - 6 * rtx_x)) / (
                    mu * sigma_ob * thick_ob * ((
                                                        rrx_x - rtx_x) ** 2 + (rrx_y - rtx_y) ** 2 + (
                                                            rtx_z - rrx_z + (2 * O) / (
                                                            mu * sigma_ob * thick_ob)) ** 2) ** (5 / 2)) - (
                                   6 * m_x * (rtx_z - rrx_z + (2 * O) / (
                                   mu * sigma_ob * thick_ob))) / (mu * sigma_ob * thick_ob * ((rrx_x - rtx_x) ** 2 + (
                    rrx_y - rtx_y) ** 2 + (rtx_z - rrx_z + (2 * O) / (mu * sigma_ob * thick_ob)) ** 2) ** (5 / 2)))
            dh_obx2 = (5 * (6 * rrx_x - 6 * rtx_x) * (
                    rtx_z - rrx_z + (2 * O) / (mu * sigma_ob * thick_ob)) * (
                               m_x * (rrx_x - rtx_x) + m_y * (rrx_y - rtx_y) - m_z * (
                               rtx_z - rrx_z + (2 * O) / (mu * sigma_ob * thick_ob))) / (
                               mu * sigma_ob * thick_ob * ((rrx_x - rtx_x) ** 2 + (
                               rrx_y - rtx_y) ** 2 + (rtx_z - rrx_z + (2 * O) / (
                               mu * sigma_ob * thick_ob)) ** 2) ** (7 / 2)))
            dh_obx = (-1 / (4 * math.pi)) * (dh_obx1 + dh_obx2)

            if rrx_z > 0:
                dh_oby = (-1 / (4 * math.pi)) * ((m_z * (6 * rrx_y - 6 * rtx_y)) / (mu * sigma_ob * thick_ob * (
                        (rrx_x - rtx_x) ** 2 + (rrx_y - rtx_y) ** 2 + (rrx_z + rtx_z + (2 * O) / (
                        mu * sigma_ob * thick_ob)) ** 2) ** (5 / 2)) - (6 * m_y * (rrx_z + rtx_z + (2 * O) / (
                        mu * sigma_ob * thick_ob))) / (mu * sigma_ob * thick_ob * ((rrx_x - rtx_x) ** 2 + (
                        rrx_y - rtx_y) ** 2 + (rrx_z + rtx_z + (2 * O) / (
                        mu * sigma_ob * thick_ob)) ** 2) ** (5 / 2)) + (5 * (6 * rrx_y - 6 * rtx_y) * (
                        rrx_z + rtx_z + (2 * O) / (mu * sigma_ob * thick_ob)) * (
                                                                                m_x * (rrx_x - rtx_x) - m_z * (
                                                                                rrx_z + rtx_z + (2 * O) / (
                                                                                mu * sigma_ob * thick_ob)) + m_y * (
                                                                                        rrx_y - rtx_y))) / (
                                                         mu * sigma_ob * thick_ob * ((rrx_x - rtx_x) ** 2 + (
                                                         rrx_y - rtx_y) ** 2 + (rrx_z + rtx_z + (2 * O) / (
                                                         mu * sigma_ob * thick_ob)) ** 2) ** (7 / 2)))

            else:
                dh_oby = (-1 / (4 * math.pi)) * ((m_z * (6 * rrx_y - 6 * rtx_y)) / (mu * sigma_ob * thick_ob * (
                        (rrx_x - rtx_x) ** 2 + (rrx_y - rtx_y) ** 2 + (rrx_z + rtx_z + (2 * O) / (
                        mu * sigma_ob * thick_ob)) ** 2) ** (5 / 2)) - (6 * m_y * (rrx_z + rtx_z + (2 * O) / (
                        mu * sigma_ob * thick_ob))) / (mu * sigma_ob * thick_ob * ((rrx_x - rtx_x) ** 2 + (
                        rrx_y - rtx_y) ** 2 + (rrx_z + rtx_z + (2 * O) / (
                        mu * sigma_ob * thick_ob)) ** 2) ** (5 / 2)) + (5 * (6 * rrx_y - 6 * rtx_y) * (
                        rrx_z + rtx_z + (2 * O) / (mu * sigma_ob * thick_ob)) * (
                                                                                m_x * (rrx_x - rtx_x) - m_z * (
                                                                                rrx_z + rtx_z + (2 * O) / (
                                                                                mu * sigma_ob * thick_ob)) + m_y * (
                                                                                        rrx_y - rtx_y))) / (
                                                         mu * sigma_ob * thick_ob * ((rrx_x - rtx_x) ** 2 + (
                                                         rrx_y - rtx_y) ** 2 + (rrx_z + rtx_z + (2 * O) / (
                                                         mu * sigma_ob * thick_ob)) ** 2) ** (7 / 2)))

        if rrx_z > 0:
            dh_obz = (-1 / (4 * math.pi)) * ((6 * m_z * (rrx_z + rtx_z + (2 * O) / (mu * sigma_ob * thick_ob))) / (
                    mu * sigma_ob * thick_ob * ((rrx_x - rtx_x) ** 2 + (rrx_y - rtx_y) ** 2 + (
                    rrx_z + rtx_z + (2 * O) / (mu * sigma_ob * thick_ob)) ** 2) ** (5 / 2)) - (6 * (
                    m_x * (rrx_x - rtx_x) - m_z * (rrx_z + rtx_z + (2 * O) / (
                    mu * sigma_ob * thick_ob)) + m_y * (rrx_y - rtx_y))) / (mu * sigma_ob * thick_ob * ((
                                                                                                                rrx_x - rtx_x) ** 2 + (
                                                                                                                    rrx_y - rtx_y) ** 2 + (
                                                                                                                    rrx_z + rtx_z + (
                                                                                                                        2 * O) / (
                                                                                                                            mu * sigma_ob * thick_ob)) ** 2) ** (
                                                                                        5 / 2)) + (
                                                         m_z * (6 * rrx_z + 6 * rtx_z + (
                                                         12 * O) / (mu * sigma_ob * thick_ob))) / (
                                                         mu * sigma_ob * thick_ob * (
                                                         (rrx_x - rtx_x) ** 2 + (rrx_y - rtx_y) ** 2 + (
                                                             rrx_z + rtx_z + (2 * O)

                                                             / (mu * sigma_ob * thick_ob)) ** 2) ** (5 / 2)) + (5 * (
                    6 * rrx_z + 6 * rtx_z + (12 * O) / (mu * sigma_ob * thick_ob)) * (rrx_z + rtx_z + (2 * O) / (
                    mu * sigma_ob * thick_ob)) * (m_x * (rrx_x - rtx_x) - m_z * (rrx_z + rtx_z + (
                    2 * O) / (mu * sigma_ob * thick_ob)) + m_y * (rrx_y - rtx_y))) / (
                                                     mu * sigma_ob * thick_ob * (
                                                         (rrx_x - rtx_x) ** 2 + (rrx_y - rtx_y) ** 2 + (
                                                         rrx_z + rtx_z + (2 * O) / (
                                                             mu * sigma_ob * thick_ob)) ** 2) ** (7 / 2)))
        else:
            dh_obz = (-1 / (4 * math.pi)) * ((6 * (m_x * (rrx_x - rtx_x) + m_y * (rrx_y - rtx_y) - m_z * (
                    rtx_z - rrx_z + (2 * O) / (mu * sigma_ob * thick_ob)))) / (mu * sigma_ob * thick_ob * (
                    (rrx_x - rtx_x) ** 2 + (rrx_y - rtx_y) ** 2 + (
                    rtx_z - rrx_z + (2 * O) / (mu * sigma_ob * thick_ob)) ** 2) ** (5 / 2)) - (m_z * (
                    6 * rtx_z - 6 * rrx_z + (12 * O) / (mu * sigma_ob * thick_ob))) / (
                                                     mu * sigma_ob * thick_ob * (
                                                         (rrx_x - rtx_x) ** 2 + (rrx_y - rtx_y) ** 2 + (
                                                         rtx_z - rrx_z + (2 * O) / (
                                                             mu * sigma_ob * thick_ob)) ** 2) ** (5 / 2)) - (
                                                     6 * m_z * (
                                                         rtx_z - rrx_z + (2 * O) / (mu * sigma_ob * thick_ob))) / (
                                                     mu * sigma_ob * thick_ob * ((rrx_x - rtx_x) ** 2 + (

                                                     rrx_y - rtx_y) ** 2 + (rtx_z - rrx_z + (2 * O) / (
                                                         mu * sigma_ob * thick_ob)) ** 2) ** (5 / 2)) - (
                                                     5 * (6 * rtx_z - 6 * rrx_z + (12 * O) / (
                                                         mu * sigma_ob * thick_ob)) * (rtx_z - rrx_z + (
                                                     2 * O) / (mu * sigma_ob * thick_ob)) * (
                                                                 m_x * (rrx_x - rtx_x) + m_y * (
                                                                 rrx_y - rtx_y) - m_z * (rtx_z - rrx_z + (2 * O) / (
                                                                     mu * sigma_ob * thick_ob)))) / (
                                                     mu * sigma_ob * thick_ob * ((rrx_x - rtx_x) ** 2 + (
                                                     rrx_y - rtx_y) ** 2 + (rtx_z - rrx_z + (2 * O) / (
                                                     mu * sigma_ob * thick_ob)) ** 2) ** (7 / 2)))
        return np.array([dh_obx, dh_obz])

    # print(dh_obdt_xyz(mtx, dipole_m, rtx, rrx, np.array([1,1,1,1,1]), mu, sigma_ob,thick_ob))
    # Purpose: This function evaluates the time-derivative of the x component of the overburden field (see eq A-5a)

    def h_ob_xyz(mtx, dipole_m, rtx, rrx, O, mu, sigma_ob, thick_ob):
        m_x = dipole_m * mtx[0]
        m_y = dipole_m * mtx[1]
        m_z = dipole_m * mtx[2]
        rtx_x = rtx[0]
        rtx_y = rtx[1]
        rtx_z = rtx[2]
        rrx_x = rrx[0]
        rrx_y = rrx[1]
        rrx_z = rrx[2]

        if rrx_z > 0:
            h_obx = (-1 / (4 * math.pi)) * (
                    m_x / ((rrx_x - rtx_x) ** 2 + (rrx_y - rtx_y) ** 2 + (rrx_z + rtx_z + (2 * O) / (
                    mu * sigma_ob * thick_ob)) ** 2) ** (3 / 2) - (
                            3 * (2 * rrx_x - 2 * rtx_x) * (m_x * (rrx_x - rtx_x) - m_z * (
                            rrx_z + rtx_z + (2 * O) / (mu * sigma_ob * thick_ob)) + m_y * (rrx_y - rtx_y))) / (
                            2 * ((
                                         rrx_x - rtx_x) ** 2 + (rrx_y - rtx_y) ** 2 + (rrx_z + rtx_z + (2 * O) / (
                            mu * sigma_ob * thick_ob)) ** 2) ** (5 / 2)))
        else:
            h_obx = (-1 / (4 * math.pi)) * (
                    m_x / ((rrx_x - rtx_x) ** 2 + (rrx_y - rtx_y) ** 2 + (rtx_z - rrx_z + (2 * O) / (
                    mu * sigma_ob * thick_ob)) ** 2) ** (3 / 2) - (3 * (2 * rrx_x - 2 * rtx_x) * (m_x * (
                    rrx_x - rtx_x) + m_y * (rrx_y - rtx_y) - m_z * (rtx_z - rrx_z + (2 * O) / (
                    mu * sigma_ob * thick_ob)))) / (2 * ((rrx_x - rtx_x) ** 2 + (rrx_y - rtx_y) ** 2 + (
                    rtx_z - rrx_z + (2 * O) / (mu * sigma_ob * thick_ob)) ** 2) ** (5 / 2)))

        if rrx_z > 0:
            h = ((rrx_x - rtx_x) ** 2 + (rrx_y - rtx_y) ** 2 + (rrx_z + rtx_z + (2 * O) / (
                    mu * sigma_ob * thick_ob)) ** 2)

            h_oby = (-1 / (4 * math.pi)) * (
                    m_y / (h ** (3 / 2)) - (
                    3 * (2 * rrx_y - 2 * rtx_y) * (m_x * (rrx_x - rtx_x) - m_z * (
                    rrx_z + rtx_z + (2 * O) / (mu * sigma_ob * thick_ob)) + m_y * (rrx_y - rtx_y))) / (
                            2 * (h ** (5 / 2))))
        else:
            h = ((rrx_x - rtx_x) ** 2 + (rrx_y - rtx_y) ** 2 + (rrx_z + rtx_z + (2 * O) / (
                    mu * sigma_ob * thick_ob)) ** 2)

            h_oby = (-1 / (4 * math.pi)) * (
                    m_y / (h ** (3 / 2)) - (
                    3 * (2 * rrx_y - 2 * rtx_y) * (m_x * (rrx_x - rtx_x) - m_z * (
                    rrx_z + rtx_z + (2 * O) / (mu * sigma_ob * thick_ob)) + m_y * (rrx_y - rtx_y))) / (
                            2 * (h ** (5 / 2))))

        if rrx_z > 0:
            h_obz = (-1 / (4 * math.pi)) * (- m_z / ((rrx_x - rtx_x) ** 2 + (rrx_y - rtx_y) ** 2 + (rrx_z + rtx_z + (
                    2 * O) / (mu * sigma_ob * thick_ob)) ** 2) ** (3 / 2) - (3 * (2 * rrx_z + 2 * rtx_z + (
                    4 * O) / (mu * sigma_ob * thick_ob)) * (m_x * (rrx_x - rtx_x) - m_z * (rrx_z + rtx_z + (
                    2 * O) / (mu * sigma_ob * thick_ob)) + m_y * (rrx_y - rtx_y))) / (2 * ((
                                                                                                   rrx_x - rtx_x) ** 2 + (
                                                                                                       rrx_y - rtx_y) ** 2 + (
                                                                                                       rrx_z + rtx_z + (
                                                                                                           2 * O) / (
                                                                                                               mu * sigma_ob * thick_ob)) ** 2) ** (
                                                                                                  5 / 2)))
        else:
            h_obz = (-1 / (4 * math.pi)) * (m_z / ((rrx_x - rtx_x) ** 2 + (rrx_y - rtx_y) ** 2 + (rtx_z - rrx_z + (
                    2 * O) / (mu * sigma_ob * thick_ob)) ** 2) ** (3 / 2) + (3 * (2 * rtx_z - 2 * rrx_z + (
                    4 * O) / (mu * sigma_ob * thick_ob)) * (m_x * (rrx_x - rtx_x) + m_y * (
                    rrx_y - rtx_y) - m_z * (rtx_z - rrx_z + (2 * O) / (mu * sigma_ob * thick_ob)))) / (
                                                    2 * (
                                                        (rrx_x - rtx_x) ** 2 + (rrx_y - rtx_y) ** 2 + (rtx_z - rrx_z + (
                                                        2 * O) / (mu * sigma_ob * thick_ob)) ** 2) ** (5 / 2)))
        return np.array([h_obx, h_obz])

    # print(h_ob_xyz(mtx, dipole_m, rtx, rrx, np.array([1,1,1,1,1]), mu, sigma_ob,thick_ob))

    # Purpose: This function evaluates the z component o f the  overburden field (see eq A-3c)
    # Contract Float Int -> Float

    # Purpose: static calculates the field of a dipole
    # m is the magnetic field vector
    # r is the vector from the dipole to the field location
    # m is the dipole moment vector
    # Static magnetic field (hx,hy,hz) at r=(x,y,z)   [ or:  Bx,By,Bz ]
    # of a magnetic dipole (mx,my,mz) at (0,0,0)
    # The units of h are the same as those of m. So if you want the B-field
    # then multiply all components of mm by mu0 before calling this routine !

    def static(m, r):
        one_over_4pi = 1 / (4 * math.pi)
        r2 = np.dot(r, r)
        if r2 < 1.e-20:
            h = 0.0
        else:
            a = one_over_4pi / (math.sqrt(r2) * r2)
            b = np.dot(r, m) * 3 / r2
            h = (b * r - m) * a
        return h

    # Purpose: This function calculates the time-dependant part of a step-response of the sphere alone
    # Equation 12-13
    # Includes the pre-factor in equation 21b

    def thetafunction_step(t, O, o, mu, sigma_sp, a, T):

        theta = 0
        solver = 0
        k = 0

        while solver < 100:
            k = k + 1

            temp = (1 / (1 + np.exp(-(T / 2) * ((k * math.pi) ** 2) / (mu * sigma_sp * (a ** 2))))) * (
                    (6 / ((k * math.pi) ** 2)) * np.exp(
                (o + O - t) * ((k * math.pi) ** 2) / (mu * sigma_sp * (a ** 2))))

            theta = theta + temp

            solver = np.linalg.lstsq(np.transpose(np.atleast_2d(temp)), np.transpose(np.atleast_2d(theta)), rcond=-1)[0]

        return theta

    def dh_tot_step(mtx, dipole_m, rtx, rsp, mu, sigma_ob, thick_ob, t, o, sigma_sp, a, T):

        import quadpy

        s = 0.0
        b = t
        n = 1
        start_points = np.linspace(s, b, n, endpoint=False)
        h = (b - s) / n
        end_points = start_points + h
        intervals = np.array([start_points, end_points])
        ob_array = h_ob_xyz(mtx, dipole_m, rtx, rsp, -o, mu, sigma_ob, thick_ob)
        thetaz = thetafunction_step(t, 0, o, mu, sigma_sp, a, T)
        scheme = quadpy.c1.gauss_kronrod(2)
        val = scheme.integrate(
            lambda O: -dh_obdt_xyz(mtx, dipole_m, rtx, rsp, O, mu, sigma_ob, thick_ob) * \
                      thetafunction_step(t, O, o, mu, sigma_sp, a, T),
            intervals
        )

        return np.array([val[0] + (ob_array[0] * thetaz), val[1] + (ob_array[1] * thetaz)])

    # Purpose: this function calculates the sphere-overburden response in the first order, no wave

    def h_total_step_1storder(
            mtx, dipole_m, rtx, offset_tx_rx, rsp, t, mu, sigma_ob, thick_ob, sigma_sp, a, apply_dip, dip, strike, T):

        # This loop convolves the tx waveform with x component

        temp = dh_tot_step(
            mtx, dipole_m, [0, 0, rtx[2]], [-rtx[0], -rtx[1], rsp[2]], mu, sigma_ob, thick_ob, t, 0, sigma_sp, a, T)

        # store x component of induced moment

        convo_y = 0
        H_tot_y = 0
        H_y = 0

        # store z component of induced moment

        # store sphere moment
        testing = temp[0][0]
        testing2 = temp[1]
        testing3 = np.array([(2 * math.pi * (a ** 3)) * temp[0][0], 0.00, (2 * math.pi * (a ** 3)) * temp[1][0]])
        msp = np.array([(2 * math.pi * (a ** 3)) * temp[0][0], convo_y, (2 * math.pi * (a ** 3)) * temp[1][0]])

        # dipping sphere model if applydip=1

        if apply_dip == 1:
            norm = np.array([(math.cos((90 - dip) * (math.pi / 180))) * (math.cos((strike - 90) * (math.pi / 180))),
                             math.sin((strike - 90) * (math.pi / 180)) * math.cos(((90 - dip)) * (math.pi / 180)),
                             math.sin((90 - dip) * (math.pi / 180))])

            # make the dip normal vector a unit vector

            normt = math.sqrt(np.dot(norm, norm))
            norm = norm / normt
            mspdotnorm = np.dot(msp, norm)

            # now scale the normal to have this strength and redirect the sphere
            # moment to be in the dip direction

            msp = mspdotnorm * norm

        # calculate field using induced moment

        static_xy = static(msp, (np.array([-offset_tx_rx[0], -offset_tx_rx[1], rtx[2] - offset_tx_rx[2]]) -
                                 (np.array([-rtx[0], -rtx[1], rsp[2]]))))

        H_tot_x = -(np.dot([1, 0, 0], static_xy))
        H_tot_z = np.dot([0, 0, 1], static_xy)
        # calculate 0th order term (field of overburden alone) and convolve with waveform

        H_field = h_ob_xyz(mtx, dipole_m, [0, 0, rtx[2]],
                           [-offset_tx_rx[0], -offset_tx_rx[1], rtx[2] - offset_tx_rx[2]],
                           t, mu, sigma_ob, thick_ob)
        # xfinal = H_tot_x + H_x

        # zfinal = H_tot_z + H_z

        final_lst = np.array([H_tot_x + H_field[0], H_tot_z - H_field[1]])

        return final_lst

    for j in list(range(0, nw)):  # iterate time
        i = -1
        #print(j)
        for x in list(range(-800, 800 + 16, int(delta_x))):  # iterate along profile
            i += 1
            profile[0, i] = x - offset_tx_rx[0]
            rtx[0] = x
            rrx[0] = rtx[0] - offset_tx_rx[0]
            # calculate response
            response_array = (h_total_step_1storder(mtx, dipole_m, rtx, offset_tx_rx, rsp, wc[j], mu, sigma_ob,
                                                    thick_ob, sigma_sp, a, apply_dip, dip, strike, T))

            H_tot_x[j, i] = (mu / 1e-12) * response_array[0]
            H_tot_z[j, i] = (mu / 1e-12) * response_array[1]


    matplotlib.rcParams["font.size"] = 14

    figure, ax = plt.subplots(2, 1,figsize=(8, 12))
    i = 0
    while i < len(H_tot_x):
        ax[0].plot(np.linspace(profile[0][0], profile[0][100], 101),
                        H_tot_x[i])  # will have to change x axis for changing param
        i += 1
    #ax[0].plot(1, 2, "b-", lw=2)
    c = 0
    while c < len(H_tot_z):
        ax[1].plot(np.linspace(profile[0][0], profile[0][100], 101),
                        H_tot_z[c])  # will have to change x axis for changing param
        c += 1
    ax[0].set_xlabel('Profile Position (m)')
    ax[0].set_ylabel('X Response (nT)')
    ax[1].set_xlabel('Profile Position(m)')
    ax[1].set_ylabel('Z Response (nT)')

    major_ticks = np.arange(-1000, 800, 200)
    minor_ticks = np.arange(-1000, 800, 100)

    ax[0].set_xticks(major_ticks)
    ax[0].set_xticks(minor_ticks, minor=True)
    ax[1].set_xticks(major_ticks)
    ax[1].set_xticks(minor_ticks, minor=True)
    #ax[0].set_yticks(major_ticks)
    #ax[0].set_yticks(minor_ticks, minor=True)

    # And a corresponding grid
    #ax[0].grid(which='minor')

    # Or if you want different settings for the grids:
    #ax[0].grid(which='minor', alpha=0.2)
    #ax[0].grid(which='major', alpha=0.5)

    #ax[0].xaxis.set_ticks([-1000,-900,-800,-700,-600,-500,-400,-300,-200,-100,0,100,200,300,400,500,600,700])
    ax[0].grid(True,which="both")
    ax[1].grid(True,which="both")
    plt.tight_layout()
    plt.show()
    return
#$tx_z$

def SphereWidget():
    i = interact(
        sphereresponse,
        aw=FloatText(min=1.0, max=300.0, step=1, value=100, continuous_update=False, description="$a$"),

        sigob=FloatText(min=0.001, max=5.0, step=.01, value=0.03, continuous_update=False, description="$\sigma_{ob}$"),

        sigsp=FloatText(min=0.01, max=10.0, step=.01, value=0.5, continuous_update=False, description="$\sigma_{sp}$"),

        thickob=FloatText(min=2.0, max=100.0, step=1, value=4, continuous_update=False, description="$t$"),

        depth=FloatText(min=-500.0, max=-10.0, step=1, value=-200.0, continuous_update=False, description="$d$"),

        txheight=FloatText(min=30.0, max=250.0, step=1, value=120.0, continuous_update=False, description="$tx_z$"),

        dipw=FloatText(min=0.0, max=360.0, step=1, value=0.0, continuous_update=False, description="$dip$"),

    )
    return i
SphereWidget()
