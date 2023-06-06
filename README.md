# Streamline Konzeption

## Clients

| Nr. | Name             | IPv4        | Subnetzmaske  | Gateway v4  | IPv6                   | Gateway v6        | VLAN    |
| --- | ---------------- | ----------- | ------------- | ----------- | ---------------------- | ----------------- | ------- |
| 0.  | Client_HH_01     | 192.168.0.2 | 255.255.255.0 | 192.168.0.1 | 2001:DB8:F1:11::2 / 64 | 2001:DB8:F1:11::1 | 11      |
| 1.  | Client_HH_02     | 192.168.1.2 | 255.255.255.0 | 192.168.1.1 | 2001:DB8:F1:12::2 / 64 | 2001:DB8:F1:12::1 | 12      |
| 2.  | Client_HL_01     | 192.168.2.2 | 255.255.255.0 | 192.168.2.1 | 2001:DB8:F2:13::2 / 64 | 2001:DB8:F2:13::1 | 13      |
| 3.  | Client_HL_02     | 192.168.3.2 | 255.255.255.0 | 192.168.3.1 | 2001:DB8:F2:14::2 / 64 | 2001:DB8:F2:14::1 | 14      |
| 4.  | Server_B_DNS_Web | 192.168.4.2 | 255.255.255.0 | 192.168.4.1 | 2001:DB8:F3:15::2 / 64 | 2001:DB8:F3:15::1 | default |
| 5.  | Server_M_Web     | 192.168.5.2 | 255.255.255.0 | 192.168.5.1 | 2001:DB8:F4:16::2 / 64 | 2001:DB8:F4:16::1 | default |

## Multilayer Switches

| Nr. | Name         |
| --- | ------------ |
| 0.  | Switch_HH_01 |
| 1.  | Switch_HL_01 |
| 2.  | Switch_B_01  |
| 3.  | Switch_M_01  |

## Router

| Nr. | Name         | OSPFv6 Router ID |
| --- | ------------ | ---------------- |
| 0.  | Router_HH_01 | 1.1.1.1          |
| 1.  | Router_HL_01 | 2.2.2.2          |
| 2.  | Router_B_01  | 3.3.3.3          |
| 3.  | Router_M_01  | 4.4.4.4          |

### 0. Router_HH_01

| Serial | IPv4     | IPv6           |
| ------ | -------- | -------------- |
| 0/1/0  | 10.0.0.2 | fd00::2 / 32   |
| 0/1/1  | 11.0.0.2 | fd00:1::2 / 32 |
| 0/2/0  | 14.0.0.2 | fd00:4::2 / 32 |
| 0/2/1  | none     | none           |

### 1. Router_HL_01

| Serial | IPv4     | IPv6           |
| ------ | -------- | -------------- |
| 0/1/0  | 10.0.0.3 | fd00::3 / 32   |
| 0/1/1  | 12.0.0.2 | fd00:2::2 / 32 |
| 0/2/0  | none     | none           |
| 0/2/1  | 15.0.0.3 | fd00:5::2 / 32 |

### Router_B_01

| Serial | IPv4     | IPv6           |
| ------ | -------- | -------------- |
| 0/1/0  | 13.0.0.2 | fd00:3::2 / 32 |
| 0/1/1  | 11.0.0.3 | fd00:1::3 / 32 |
| 0/2/0  | none     | none           |
| 0/2/1  | 15.0.0.2 | fd00:5::3 / 32 |

### Router_M_01

| Serial | IPv4     | IPv6           |
| ------ | -------- | -------------- |
| 0/1/0  | 13.0.0.3 | fd00:3::3 / 32 |
| 0/1/1  | 12.0.0.3 | fd00:2::3 / 32 |
| 0/2/0  | 14.0.0.3 | fd00:4::3 / 32 |
| 0/2/1  | none     | none           |

## SSH

### SSH Banner

The banner is to be displayed on all layer 3 devices

```txt
***************************************************************
*                                                             *
*  UNAUTHORIZED ACCESS TO THIS DEVICE IS PROHIBITED           *
*                                                             *
*  This device belongs to Techniker Krankenkasse.             *
*  If you are not an authorized user, disconnect immediately. *
*                                                             *
***************************************************************
```

### SSH passwords

| Device       | Password            |
| ------------ | ------------------- |
| Router_HH_01 | hh_01_router_secret |
| Router_HL_01 | hl_01_router_secret |
| Router_B_01  | b_01_router_secret  |
| Router_M_01  | m_01_router_secret  |
| Switch_HH_01 | hh_01_switch_secret |
| Switch_HL_01 | hl_01_switch_secret |
| Switch_B_01  | b_01_switch_secret  |
| Switch_M_01  | m_01_switch_secret  |

## TLDR config

### Cisco router

#### Configure truncated vlan for interface

```bash
enable
configure terminal
interface <interface_identifier>.<vlan>
encapsulation dot1Q <vlan>
ip address <ipv4_address> <subnetmask>
ipv6 address <ipv6_address>/<suffix>
no shutdown
description sub-interface fuer vlan<vlan>
```

#### Configure routing networks for interface

```bash
enable
configure terminal
interface <interface_identifier>
ip address <ipv4_address> <subnetmask>
ipv6 address <ipv6_address>/<suffix>
no shutdown
```

#### Configure ipv4 routing via RIPv2

```bash
enable
configure terminal
router rip
version 2
network <network_address>
```

#### Configure OSPFv6 routing

```bash
enable
configure terminal
ipv6 router ospf <process_id>
router-id <router_id>
exit
interface <interface_identifier>
ipv6 ospf <process_id> area <area_id>
```

#### Generate ssh keys

```bash
enable
configure terminal
crypto key generate rsa general-keys modulus <key-size>
```

#### Set ssh password
```bash
enable
configure terminal
line vty 0 15
transport input ssh
password <your-password>
login
```

#### Save Configuration

```bash
enable
copy run start
```
