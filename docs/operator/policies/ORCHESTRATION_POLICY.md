# Provider Orchestration Policy

1. Operator builds an ordered `OperatorPlan`.  
2. Steps run sequentially unless a future policy marks parallel-safe reads.  
3. On step failure: stop subsequent steps; compose truthful failure.  
4. Multi-domain plans require a composition id or explicit Operator plan.  
5. Providers never orchestrate other providers.
