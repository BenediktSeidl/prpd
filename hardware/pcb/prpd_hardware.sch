EESchema Schematic File Version 4
EELAYER 30 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 1 1
Title ""
Date ""
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L Connector:RJ45_Shielded J1
U 1 1 600C85C2
P 3250 2000
F 0 "J1" H 3307 2667 50  0000 C CNN
F 1 "RJ45_Shielded" H 3307 2576 50  0000 C CNN
F 2 "prpd_hardware:CAT5 T1U 2.8N4N" V 3250 2025 50  0001 C CNN
F 3 "~" V 3250 2025 50  0001 C CNN
	1    3250 2000
	1    0    0    -1  
$EndComp
$Comp
L Connector:RJ45_Shielded J11
U 1 1 600C9484
P 6400 2000
F 0 "J11" H 6457 2667 50  0000 C CNN
F 1 "RJ45_Shielded" H 6457 2576 50  0000 C CNN
F 2 "prpd_hardware:CAT5 T1U 2.8N4N" V 6400 2025 50  0001 C CNN
F 3 "~" V 6400 2025 50  0001 C CNN
	1    6400 2000
	-1   0    0    -1  
$EndComp
$Comp
L Connector:Conn_01x01_Male T8
U 1 1 600D90A9
P 4450 1400
F 0 "T8" V 4600 1400 50  0000 L CNN
F 1 " " V 4603 1444 50  0000 L CNN
F 2 "prpd_hardware:tab" H 4450 1400 50  0001 C CNN
F 3 "~" H 4450 1400 50  0001 C CNN
	1    4450 1400
	0    1    1    0   
$EndComp
$Comp
L Connector:Conn_01x01_Male T7
U 1 1 600DA03B
P 4600 1500
F 0 "T7" V 4750 1500 50  0000 L CNN
F 1 " " V 4753 1544 50  0000 L CNN
F 2 "prpd_hardware:tab" H 4600 1500 50  0001 C CNN
F 3 "~" H 4600 1500 50  0001 C CNN
	1    4600 1500
	0    1    1    0   
$EndComp
$Comp
L Connector:Conn_01x01_Male T6
U 1 1 600DA6BC
P 4750 1600
F 0 "T6" V 4900 1600 50  0000 L CNN
F 1 " " V 4903 1644 50  0000 L CNN
F 2 "prpd_hardware:tab" H 4750 1600 50  0001 C CNN
F 3 "~" H 4750 1600 50  0001 C CNN
	1    4750 1600
	0    1    1    0   
$EndComp
$Comp
L Connector:Conn_01x01_Male T5
U 1 1 600DAA20
P 4900 1700
F 0 "T5" V 5050 1700 50  0000 L CNN
F 1 " " V 5053 1744 50  0000 L CNN
F 2 "prpd_hardware:tab" H 4900 1700 50  0001 C CNN
F 3 "~" H 4900 1700 50  0001 C CNN
	1    4900 1700
	0    1    1    0   
$EndComp
$Comp
L Connector:Conn_01x01_Male T4
U 1 1 600DAF87
P 5050 1800
F 0 "T4" V 5200 1800 50  0000 L CNN
F 1 " " V 5203 1844 50  0000 L CNN
F 2 "prpd_hardware:tab" H 5050 1800 50  0001 C CNN
F 3 "~" H 5050 1800 50  0001 C CNN
	1    5050 1800
	0    1    1    0   
$EndComp
$Comp
L Connector:Conn_01x01_Male T3
U 1 1 600DB38E
P 5200 1900
F 0 "T3" V 5350 1900 50  0000 L CNN
F 1 " " V 5353 1944 50  0000 L CNN
F 2 "prpd_hardware:tab" H 5200 1900 50  0001 C CNN
F 3 "~" H 5200 1900 50  0001 C CNN
	1    5200 1900
	0    1    1    0   
$EndComp
$Comp
L Connector:Conn_01x01_Male T2
U 1 1 600DB7D2
P 5350 2000
F 0 "T2" V 5500 2000 50  0000 L CNN
F 1 " " V 5503 2044 50  0000 L CNN
F 2 "prpd_hardware:tab" H 5350 2000 50  0001 C CNN
F 3 "~" H 5350 2000 50  0001 C CNN
	1    5350 2000
	0    1    1    0   
$EndComp
$Comp
L Connector:Conn_01x01_Male T1
U 1 1 600DBB38
P 5500 2100
F 0 "T1" V 5650 2100 50  0000 L CNN
F 1 " " V 5653 2144 50  0000 L CNN
F 2 "prpd_hardware:tab" H 5500 2100 50  0001 C CNN
F 3 "~" H 5500 2100 50  0001 C CNN
	1    5500 2100
	0    1    1    0   
$EndComp
$Comp
L Interface_UART:SP3485EN U1
U 1 1 600DC9C0
P 5050 3050
F 0 "U1" V 5096 3494 50  0000 L CNN
F 1 "SP3485EN" V 5005 3494 50  0000 L CNN
F 2 "Package_SO:SOIC-8_3.9x4.9mm_P1.27mm" H 6100 2700 50  0001 C CIN
F 3 "http://www.icbase.com/pdf/SPX/SPX00480106.pdf" H 5050 3050 50  0001 C CNN
	1    5050 3050
	0    1    -1   0   
$EndComp
$Comp
L Connector_Generic:Conn_01x03 J2
U 1 1 600E1A84
P 4050 3850
F 0 "J2" H 3968 4167 50  0000 C CNN
F 1 "Conn_01x03" H 3968 4076 50  0000 C CNN
F 2 "Connector_JST:JST_XH_B3B-XH-A_1x03_P2.50mm_Vertical" H 4050 3850 50  0001 C CNN
F 3 "~" H 4050 3850 50  0001 C CNN
	1    4050 3850
	-1   0    0    -1  
$EndComp
Wire Wire Line
	3650 1600 4450 1600
Wire Wire Line
	6000 1700 4600 1700
Wire Wire Line
	3650 1800 4750 1800
Wire Wire Line
	6000 1900 4900 1900
Wire Wire Line
	3650 2000 5050 2000
Wire Wire Line
	6000 2100 5200 2100
Wire Wire Line
	3650 2200 5350 2200
Wire Wire Line
	6000 2300 5500 2300
Connection ~ 4450 1600
Wire Wire Line
	4450 1600 6000 1600
Connection ~ 4600 1700
Wire Wire Line
	4600 1700 3650 1700
Connection ~ 4750 1800
Wire Wire Line
	4750 1800 6000 1800
Connection ~ 4900 1900
Wire Wire Line
	4900 1900 3650 1900
Connection ~ 5050 2000
Wire Wire Line
	5050 2000 6000 2000
Connection ~ 5200 2100
Wire Wire Line
	5200 2100 3650 2100
Connection ~ 5350 2200
Wire Wire Line
	5350 2200 6000 2200
Connection ~ 5500 2300
Wire Wire Line
	3650 2300 4550 2300
Wire Wire Line
	5150 2650 5200 2650
Wire Wire Line
	5200 2650 5200 2550
Wire Wire Line
	4950 2650 4750 2650
Wire Wire Line
	4750 2650 4750 2550
Wire Wire Line
	4550 2300 4550 2700
Wire Wire Line
	4550 3050 4650 3050
Connection ~ 4550 2300
Wire Wire Line
	4550 2300 5500 2300
Wire Wire Line
	4250 3750 4300 3750
Wire Wire Line
	4550 3750 4550 3650
Connection ~ 4550 3050
Wire Wire Line
	4250 3950 4300 3950
Wire Wire Line
	5450 3950 5450 3050
Wire Wire Line
	4250 3850 5250 3850
Wire Wire Line
	5250 3850 5250 3450
Wire Wire Line
	5150 3450 5150 3750
Wire Wire Line
	5150 3750 4550 3750
Connection ~ 4550 3750
Wire Wire Line
	4850 3450 4850 3550
Wire Wire Line
	4850 3550 4550 3550
Connection ~ 4550 3550
Wire Wire Line
	4550 3550 4550 3050
Wire Wire Line
	4950 3450 4950 3650
Wire Wire Line
	4950 3650 4550 3650
Connection ~ 4550 3650
Wire Wire Line
	4550 3650 4550 3550
Wire Wire Line
	6400 2500 3250 2500
$Comp
L Connector:Conn_01x01_Male TI2
U 1 1 6017E737
P 5000 2550
F 0 "TI2" V 5150 2550 50  0001 L CNN
F 1 "A-" V 5153 2594 50  0000 L CNN
F 2 "prpd_hardware:tab" H 5000 2550 50  0001 C CNN
F 3 "~" H 5000 2550 50  0001 C CNN
	1    5000 2550
	1    0    0    -1  
$EndComp
$Comp
L Connector:Conn_01x01_Male TI3
U 1 1 6017F0DA
P 4950 2550
F 0 "TI3" V 5100 2550 50  0001 L CNN
F 1 "B+" V 5103 2594 50  0000 L CNN
F 2 "prpd_hardware:tab" H 4950 2550 50  0001 C CNN
F 3 "~" H 4950 2550 50  0001 C CNN
	1    4950 2550
	-1   0    0    1   
$EndComp
$Comp
L Connector:Conn_01x01_Male TI1
U 1 1 6017F6C1
P 4350 2700
F 0 "TI1" V 4500 2700 50  0001 L CNN
F 1 "GND" V 4503 2744 50  0000 L CNN
F 2 "prpd_hardware:tab" H 4350 2700 50  0001 C CNN
F 3 "~" H 4350 2700 50  0001 C CNN
	1    4350 2700
	1    0    0    -1  
$EndComp
Connection ~ 4550 2700
Wire Wire Line
	4550 2700 4550 3050
Connection ~ 4750 2550
Wire Wire Line
	4750 2550 4750 1800
Connection ~ 5200 2550
Wire Wire Line
	5200 2550 5200 2100
$Comp
L Device:C C1
U 1 1 60196B0E
P 3700 3850
F 0 "C1" H 3815 3896 50  0000 L CNN
F 1 "0.1uF" H 3815 3805 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric" H 3738 3700 50  0001 C CNN
F 3 "~" H 3700 3850 50  0001 C CNN
	1    3700 3850
	1    0    0    -1  
$EndComp
Wire Wire Line
	4300 3750 4300 3500
Wire Wire Line
	4300 3500 3700 3500
Wire Wire Line
	3700 3500 3700 3700
Connection ~ 4300 3750
Wire Wire Line
	4300 3750 4550 3750
Wire Wire Line
	3700 4000 3700 4150
Wire Wire Line
	3700 4150 4300 4150
Wire Wire Line
	4300 4150 4300 3950
Connection ~ 4300 3950
Wire Wire Line
	4300 3950 5450 3950
$EndSCHEMATC
