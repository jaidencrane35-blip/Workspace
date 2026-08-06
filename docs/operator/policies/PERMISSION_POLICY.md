# Permission Decision Policy

1. Operator decides whether a plan should proceed for the user turn.  
2. Kernel Permission Gateway remains the enforcement point at effect time.  
3. If Gateway denies: Operator reports truthful denial — does not retry around policy.  
4. Future ApprovalRequired flows surface through Operator replies / satellites — never silent.
