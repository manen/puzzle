# `utils/deploy`

a rather simple deploy tool, comes with a self-installing [daemon (deployd)](d/README.md) and a [ctl (deployctl)](ctl/README.md)

---

deployd and deployctl currently do not block outside traffic and will deploy any executable on your machine if you let it. it's recommended to have your firewall block port `6789` from outside traffic

---

port: `6789`
