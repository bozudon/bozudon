BEGIN TRANSACTION;

INSERT INTO account (username, display_name, note, uri, inbox_url, outbox_url, shared_inbox_url, followers_url, private_key, public_key)
VALUES
    (
        'user1',
        'User 1',
        'Hi! I am User 1',
        'https://localhost:8080/users/user1',
        'https://localhost:8080/users/user1/inbox',
        'https://localhost:8080/users/user1/outbox',
        'https://localhost:8080/inbox',
        'https://localhost:8080/users/user1/followers',
        '-----BEGIN PRIVATE KEY-----
MIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQCbBdBKqeBtLCe2
UGD/U2U9m3HhVYanYlRAwuK5osU743cApFaN4yih+T2K+P6FCHcrfbsKxfgw5Te6
TSh51QVkAPgHj4oyuct5M2meJNy5eV4Rf3iDdYa1pV4HT3R1JC1EBQmaAbT0Nx40
4kUp1r778qZJaixPTeVSCwn09b4Xl336URy310i16j+L6TZKbZvX+Djio+uRbcF7
bYM6OERMeqNNk2fadgVfUfLNBsDwzvWEp4YfC5elvbv8UMEs5BiGptLFKWbozkMI
nRg3inV1eTjEd69JTURepoylPkW1TjEhXRQ+U6BC/iDRBWEDYIGcvi3+fCHeb6CO
nWBVXBCPAgMBAAECggEADJlFW1Wnbdf0n9/nPvjtHAh1Ed+2ImzFKI1uIn4bBdbJ
Rh3h0IUEn8O2Zh2XhpvjuEu7yfb0e8ZoaAiuegnlw6hC5oJnTd+rRkqQxXYWrJaZ
6yKIwQGDlPs/XNxVPGvxp24gk9aaeTqKU7i/IqGKqSxuqgrDiewGIuVoxfK7pAXe
2/5M0nZ3v8X5HV9rZJDzVSy6IqKycfF/eqFMjS17V/wIqiCYbcpy76385m50g1My
b+d/yyWH48TfW9fX39/mONVhAZ5ihtt+GPxnE+bqU2WDFhBFxQ55DTSQ0zi0q2ox
WI/2SmTchplChZfn7ap22k/3EN+RkFpm5+Y5EliU8QKBgQDI+448d5NsJlQhOB14
2IwftjQm0//0nsm78ncVBromWBxfqWRoWrdGm+u2KBPvKjE8qOg1fYCbS+zikPFC
D2qrnD0RSCQycX/73r+/ws+afeHKRiD3YbaIs4NKdQ94CewLCSUmeLSURdXwSSqK
X15sMTwsYXdkUWvScKUWBhxB+QKBgQDFdXtiaTE7NQSObOrbxitHLaKJHT2pJiYt
utXOKaDVM4aY8CavXexfDS6DGwtr4WlVd1gWX/otCQCZjj+V+AYdVYT/KiEL4Obu
rGSOkkLMjVa7rSLYhkQk0d36K1ddgq33oNJT9XAccXaxJe1lH7AA3mjSWFe1DBa/
zurLkz0IxwKBgHpI7HMIAk/ERl3r6aq0fxVwY+zYAp0Q3AeZ8DB/5lUcOS9PPoNr
5qV0iwdK/U4AavLQhnC9SrmyiZAUxmX+QVXm+xT/wt5aRpe2IQ8I9g221+Gdp8M8
1bX5G0H7VY0g5FiWmN7+hEjO8OuBaPvGKQpFLqqGJwGHtMXWuetbjmfpAoGBALft
hO7YOWmDKylXvzTUVL/KfFnoCOWkiW+TV1DgadXuVvPizsYPYPvxFWA+MtdccneP
4VeGM0z707k1TXluPJPaczYTkhC0f6fWoRxElUBgb2gGEC1Mc/EwI+rBsHGEJnRB
M2nNd46nCf5c69KEP7evdEhqzdfw2Mf1/7N9BR9FAoGBAKXqTteRdAIzrs1ncB6P
JmP1PrF82rdpVh2T8O6BWuOL90eJh0JTjw3tkOdO2BfQvc+6xJC4iNWxHar7w+RQ
4xLi8wUCw9wFhPQnAkiBCZWf7AQS8hDhRrzC84Q5KQpdTqIzYG+W1OEsft/3RE2Z
i5o+2GJTJHWRfhSFW91fbXHG
-----END PRIVATE KEY-----
',
        '-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAmwXQSqngbSwntlBg/1Nl
PZtx4VWGp2JUQMLiuaLFO+N3AKRWjeMoofk9ivj+hQh3K327CsX4MOU3uk0oedUF
ZAD4B4+KMrnLeTNpniTcuXleEX94g3WGtaVeB090dSQtRAUJmgG09DceNOJFKda+
+/KmSWosT03lUgsJ9PW+F5d9+lEct9dIteo/i+k2Sm2b1/g44qPrkW3Be22DOjhE
THqjTZNn2nYFX1HyzQbA8M71hKeGHwuXpb27/FDBLOQYhqbSxSlm6M5DCJ0YN4p1
dXk4xHevSU1EXqaMpT5FtU4xIV0UPlOgQv4g0QVhA2CBnL4t/nwh3m+gjp1gVVwQ
jwIDAQAB
-----END PUBLIC KEY-----
'
    ),
    (
        'user2',
        'User 2',
        '',
        'https://localhost:8080/users/user2',
        'https://localhost:8080/users/user2/inbox',
        'https://localhost:8080/users/user2/outbox',
        'https://localhost:8080/inbox',
        'https://localhost:8080/users/user2/followers',
        '-----BEGIN PRIVATE KEY-----
MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCscuYF4Ti+GDn1
mZ4Giig7FQdaLdEP+sUtcVfkhkXrZrS+ZAu3g29mhfmF/F3lcMDjCbUoDvOu5EZD
SlwThE4bc2TsoGPb7/JsnvUMDjHmB33iUWtMMfRSx+mpWj4dE+wWdOipBov4OFbC
6xhDewTc5oFzQj7haaUXkIc/rnQMoiQbE+0BuuQZcuUjlCNrhM5lcSBa6LTX6Bie
YHj7s6+YfO0SWWmwIDTtF25qYmVktT1OPdS6Q+64dO4h4fCYRuCzeBdJNW5dqmnx
rPkIOi+QhM1hChSUp8EErKcHTkrl6ee1fazpgckGOAK0oX9n+AxhtCRUZalaDTgP
4PKugtyTAgMBAAECggEANmFTNRZ1OMjV+iNzqDdIAX2boG3jR+k55Z8g6tEG2nk8
VO+inD+n0Qo9ielvyKsaQF4u0M+KO6QDnjOkPolRwilM8auQYepUJbmop6EQTiwd
n9EyB0iXdcVR41QwUyYVpG1AcxC78c3+WiAduiXgfjJ4Ltr8jkOsynYKmGPQd6zj
Kv8C4rF0XOzZHfNV4kXoGxad+pLb+ASRmYXCL3Lk9x0041lBYP95qSX4otNgU6Hk
m//Jmg9hgJW//PBhzRSvRZtb57DnJvc0RurazzUxugNrszrqOfTIGxgKcGguD/eC
+gWiZJdFdo0pBpHej/HZv+eZ+XK9XUjAg1oa4Ae4kQKBgQDMn7QS5NtrcJ62rfpj
Lkj27Ww7nOLnaqL8gDx/Kgd82yj/dsFS40BNxvUBpD3ThVI+dAJdQT4glrjK/KCT
FxgmdeEMr1bnEyZE6drM+I/ZqLSZXD99AyVq1YGDHc16NOw1HJl8ky2Wf5g1V77M
smEzKjyUaT5rEPzj7JsQ87OIowKBgQDXvyFSqZZXwrWSe3dtlZNdavdowz1bxpy1
7BeBbOCF1TCM4B/eVhh5KVjpo7M5nrKyrCv5qgB6WfUPzoi/694QW+TG8QpTSo+y
6w5ogotQCaDmaTtF0hiLBGZgHVLT/zcZweRAdmp4CFqTTRsFYj/QS3MOA2+MuIjj
upfwHA3rUQKBgA+KHVHGAYfSQKtc4QQBMtdVxn9sdarfZUtJrc52caUgu6dS7HGQ
AoUlk9uxDmTu3gUSKE/OsZol+BiqQMOK4HGjSBj/g9j+kkxvegdQ5RPBhR2UbNng
kEnhtvkvHTinpjyOVHWqc67XN8btR/choNIS5hDbQTs+SgZBJLGP4FWDAoGBAJek
T9e6O2IUCWucPKVZrOrMdHm+QTJwt1VnTF48GNP4tNdP8RZljW4cEdKmrSKi2Rec
zIv/Ybad5BHoKVY8gykcbgEnCwrgLYCklsv+dc0b92v8Kx04Puf3f72u3rkDeccw
8S4eDVexB3jhiUyPcisF3SjMYWuXiSAH1yN9lIsBAoGAUf1oTaeI0RlTH/CWAXc8
10lUP3ipEZ76RMPXs1/L9FZfBWPtK1GAjKRDLM9Pzuc2To2AxbkuGSYQBcKwh3l5
WGgBxWQ/Lnfu81vIt08URl8OEqyhoXWDfqtXHI+//OnlrS8enbD2DGLj7ga+sRoE
s6CY8jTB3mhMUz4NAU4CBTE=
-----END PRIVATE KEY-----
',
        '-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEArHLmBeE4vhg59ZmeBooo
OxUHWi3RD/rFLXFX5IZF62a0vmQLt4NvZoX5hfxd5XDA4wm1KA7zruRGQ0pcE4RO
G3Nk7KBj2+/ybJ71DA4x5gd94lFrTDH0UsfpqVo+HRPsFnToqQaL+DhWwusYQ3sE
3OaBc0I+4WmlF5CHP650DKIkGxPtAbrkGXLlI5Qja4TOZXEgWui01+gYnmB4+7Ov
mHztEllpsCA07RduamJlZLU9Tj3UukPuuHTuIeHwmEbgs3gXSTVuXapp8az5CDov
kITNYQoUlKfBBKynB05K5enntX2s6YHJBjgCtKF/Z/gMYbQkVGWpWg04D+DyroLc
kwIDAQAB
-----END PUBLIC KEY-----
'
    ),
    (
        'user3',
        'User 3',
        '',
        'https://localhost:8080/users/user3',
        'https://localhost:8080/users/user3/inbox',
        'https://localhost:8080/users/user3/outbox',
        'https://localhost:8080/inbox',
        'https://localhost:8080/users/user3/followers',
        '-----BEGIN PRIVATE KEY-----
MIIEvAIBADANBgkqhkiG9w0BAQEFAASCBKYwggSiAgEAAoIBAQCL15+Tu5KeVuSI
8AAnB+9oU5xsVUKubvloZ5xuuLTWAKlXEZAs+QI8L6+nVrg6OKru7fB7z+GjO9Vk
YCq31X8AkkPAgh4IXeuF6N5b18FNTlLT39OPShb2oA5ds5mVyATAHSVd4xZnSH+g
4Q9FpOT5NNaHJTFB1CidIuCQBPu8e9py8kREopuMVkdg1xHtTuhvWfCid55yhToN
HbdgMvU1Imt5WBNSmtBRoLSDomkjh6QgqyIxC+U/at1DuSKU791PTNyrF1kg+o2V
uJ1RnCfgZikyQrOfnCFrRMOb7HnHNBE/oEYYE0niaLwKaz+aFACCwhsZfBaZO4Oq
4HujvfmVAgMBAAECggEANsSc/9vvgUnguWzdcmveLIrKcXc25WAbF/O0RXzbfhDG
QY5kW+iuImo+rhf9kPfOokLX83WoFikB2wz6zgK0ecHO/R84qeg2rLxWEbw/PYqW
T9qSXcUTl0V6OuJzHUE38xG3J70fchXnvldGhu0crCkwd79uOizgNyKItn7tJJT8
9Waooz6uPqqSfCtJ56IYjzvkl8G9Lf3Ln89K0/Sv9jcmQ5p/+icAkmxxpfDPLdFV
cL7WVCa/oyg46ggXAoamo7r3T/VuwcIIMJpn9YKQg5Eiw2Uh1liVJloJ5JhD9aFR
p7EvJqdXByTwoPmdFP+lxyLaLl237UCVuv4B1OJWpQKBgQC9kXt85ctaGoqCEzEE
c+3XOY7PnArmQYqpfqDachNxSwldKyGaYR+rxESfmYitjNhwKMPbX5R9W4nZi6Wb
jyGGMA6ZdFbpYKD0oA71FslxHmCtWsbbRlx1018yoSNWjwo9ZK9+PQo5u20Gn+Hz
VoG8OOfzBRZF3edeD9zaSQ6l5wKBgQC82SEus3TJ2BSCQui1Gy+I8YcRoP8v0ig9
LcMAyKp+DP7UaNXhIlLKiJUAg5WRRekg+i9HBx2A02nYf+c7okKLZsKOTt8CZUpZ
EgrgiIO4bBb+n8fVeznANcWMNK69ud8CsUqJj22pubrAPRpRWsDd9V3yzpJLx65m
np4MQ/j9IwKBgF4TsQEO2Zhhy5M6Cv12uvYwrlsBybbzl+j92r4OFvAGmSMPoKGI
ybrosFW0UEwwtckTsf9Qs9RydTrtPsawhaaeeuBVCDzRo49DT6j6IgZtWPUvM7jm
dkRTHc92gJ8YZbCKIz229Tgpd7Xt6qhkqgXLB/Fm+OK7eXMI1EXQM+DJAoGAALBO
BayT3XSTRpJV5OsvdjFjS7YpmBQNH4P+NQ+GR6VmfIHRXdvRL3nmCTVxozD9E6i1
6W1mNyUOOsLkpfwGgBmk1f4FpC2YYtDFB2KYEGEciFsu0GF9qWzIxqUeigSWgp0A
55WFUdwiiTbv1KIfF/AgIpWMRQh6Y1dqviK1Ur8CgYAY9CbHoFjFjLKH5Tcs995H
Iy71Qgkl6c8GSljzBl3wbUnNx7+FlP7D2IdlboQ34/kdHk9V82T9yS7pcfqYJpFq
EPWdxehnFULr5WApgbZ+5lwJrTH4mGj2iZo05y8bpovCp3m4OoNKL/ZszQBTTOSW
MqhPsUfS8Ln/cVwIKxq0Hw==
-----END PRIVATE KEY-----
',
        '-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAi9efk7uSnlbkiPAAJwfv
aFOcbFVCrm75aGecbri01gCpVxGQLPkCPC+vp1a4Ojiq7u3we8/hozvVZGAqt9V/
AJJDwIIeCF3rhejeW9fBTU5S09/Tj0oW9qAOXbOZlcgEwB0lXeMWZ0h/oOEPRaTk
+TTWhyUxQdQonSLgkAT7vHvacvJERKKbjFZHYNcR7U7ob1nwoneecoU6DR23YDL1
NSJreVgTUprQUaC0g6JpI4ekIKsiMQvlP2rdQ7kilO/dT0zcqxdZIPqNlbidUZwn
4GYpMkKzn5wha0TDm+x5xzQRP6BGGBNJ4mi8Cms/mhQAgsIbGXwWmTuDquB7o735
lQIDAQAB
-----END PUBLIC KEY-----
'
    );

INSERT INTO user_ (email, encrypted_password, account_id)
VALUES
    (
        'user1@example.com',
        '$2y$10$5nGAZW6/iPChMaraRQ3Wqu7.T3qUctRvtZZ.FfcBF/ot5wi9W2f9q', -- "password"
        1
    ),
    (
        'user2@example.com',
        '$2y$10$9D4m7pZy74MEiXRLUeMdLuSETDS0ri.xUnhcLYbeGBAzQUXmfPfaO', -- "password"
        2
    ),
    (
        'user3@example.com',
        '$2y$10$KZ327gRc0FUON5BZAVL3pe5xIIUzm/p8iY9Rd92B8NAHYIXzfNkwm', -- "password"
        3
    );

COMMIT;
