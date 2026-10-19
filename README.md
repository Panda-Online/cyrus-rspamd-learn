# cyrus-rspamd-learn
Script handles training of rspamd Bayesian filters via Junk folder. 

The cyrus-rspamd-learn script is a one-shot program that:

1. takes an event JSON payload from stdin
2. ignores any move that are not in/out of the Junk folder
3. figures whether the action is learm_spam or learn_ham (depending on the source / destination mailbox)
4. figures the file path to the relevant messag(s) (using Cyrus' mbpath command)
5. calls rspamc with the right arguments and file path

In addition, the script logs information so that it can be debugged / observed
