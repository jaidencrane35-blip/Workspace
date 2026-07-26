/** AUTO-GENERATED — do not edit. Source: packages/kernel/resources/explanation-catalog.json
 * Regenerate: node scripts/sync-explanation-catalog.mjs
 * Verify: node scripts/verify-explanation-catalog.mjs
 */
export default {
  "version": 2,
  "resolution": {
    "order": [
      "exact",
      "prefix_suffix",
      "prefix_pattern",
      "prefix_fallback",
      "unknown"
    ],
    "unknown": {
      "description_template": "No Experience translation for '{explanation_key}' yet (signal {signal} from {source}, weight {weight}).",
      "signal_title_fallback_template": "Attention signal '{signal}' needs attention"
    }
  },
  "exact": {
    "decision.base.blocker": {
      "title": "Blocked decision needs attention",
      "description": "because a blocked Decision Queue item is stopping progress"
    },
    "decision.base.outstanding": {
      "title": "Outstanding decision needs attention",
      "description": "because a pending Decision Queue item still needs a human choice"
    },
    "decision.priority.critical": {
      "title": "Critical decision raises focus",
      "description": "because this Decision Queue item is marked critical"
    },
    "decision.priority.high": {
      "title": "High-priority decision raises focus",
      "description": "because this Decision Queue item is marked high priority"
    },
    "decision.priority.normal": {
      "title": "Decision contributes to focus",
      "description": "because this Decision Queue item carries normal priority"
    },
    "decision.deferred": {
      "title": "Deferred decision still matters",
      "description": "because a deferred Decision Queue item remains unresolved"
    },
    "continuity.interrupted": {
      "title": "Interrupted work needs attention",
      "description": "because Continuity shows work that was left unfinished"
    },
    "continuity.resumable": {
      "title": "Resumable work is ready",
      "description": "because Continuity can pick up where you left off"
    },
    "continuity.commitment": {
      "title": "Pending commitment needs attention",
      "description": "because an automation commitment is still awaiting resolution"
    },
    "continuity.dormant": {
      "title": "Dormant work needs attention",
      "description": "because Continuity shows work that has gone quiet"
    },
    "continuity.focus": {
      "title": "Current focus needs attention",
      "description": "because Continuity identifies this as the active focus"
    },
    "activity.progress": {
      "title": "Recent progress is worth noticing",
      "description": "because Activity Graph recorded related work recently"
    },
    "purpose.outcome": {
      "title": "Purpose progress is visible",
      "description": "because Purpose reports meaningful progress toward the goal"
    },
    "evolution.insight": {
      "title": "Workspace evolution needs attention",
      "description": "because Evolution surfaced how work recently changed"
    },
    "recommendation.candidate": {
      "title": "A next-step suggestion is available",
      "description": "because the Recommendation Engine proposed a candidate action"
    },
    "pattern.observation": {
      "title": "A work pattern was observed",
      "description": "because Pattern Model noticed a recurring workspace signal"
    }
  },
  "prefix_rules": [
    {
      "prefix": "task.base.",
      "suffixes": {
        "blocked": {
          "title": "Blocked task needs attention",
          "description": "because work is currently waiting on completion"
        },
        "waiting": {
          "title": "Waiting task needs attention",
          "description": "because this task is waiting on a dependency"
        },
        "in_progress": {
          "title": "In-progress task needs attention",
          "description": "because active work is underway and still open"
        },
        "ready": {
          "title": "Ready task needs attention",
          "description": "because open work is ready to continue"
        }
      },
      "fallback": {
        "title": "Open task needs attention",
        "description": "because Task Graph still lists this work as open"
      }
    },
    {
      "prefix": "task.priority.",
      "suffixes": {
        "critical": {
          "title": "Critical priority raises focus",
          "description": "because this task is marked critical"
        },
        "high": {
          "title": "High priority raises focus",
          "description": "because this task is marked high priority"
        },
        "normal": {
          "title": "Normal priority contributes to focus",
          "description": "because this task carries a normal priority band"
        },
        "low": {
          "title": "Low priority still contributes",
          "description": "because open low-priority work remains on the graph"
        }
      },
      "fallback": {
        "title": "Task priority contributes to focus",
        "description": "because Task Graph priority influenced this ranking"
      }
    },
    {
      "prefix": "purpose.obstacle.",
      "suffixes": {
        "blocked_task": {
          "title": "Purpose blocked by a task",
          "description": "because a blocked Task Graph node sits on the path to Purpose"
        },
        "blocked_work": {
          "title": "Purpose blocked by open work",
          "description": "because blocked Continuity work is stalling Purpose progress"
        },
        "interrupted_work": {
          "title": "Purpose interrupted",
          "description": "because interrupted work is pulling focus away from Purpose"
        },
        "outstanding_decisions": {
          "title": "Purpose waiting on decisions",
          "description": "because outstanding decisions gate Purpose progress"
        }
      },
      "patterns": [
        {
          "starts_with": "composition:",
          "title": "Purpose blocked by composition",
          "description": "because a Composition gap is obstructing Purpose progress"
        }
      ],
      "fallback": {
        "title": "Purpose obstacle needs attention",
        "description": "because Purpose reports an obstacle on the current path"
      }
    },
    {
      "prefix": "composition.gap.",
      "suffixes": {
        "disconnected_work": {
          "title": "Composition is disconnected from work",
          "description": "because the working environment does not match active work"
        },
        "missing_application": {
          "title": "Composition is missing an application",
          "description": "because a required application is not present in the composition"
        }
      },
      "fallback": {
        "title": "Composition gap needs attention",
        "description": "because Composition reports a membership gap"
      }
    },
    {
      "prefix": "environment.gap.",
      "suffixes": {
        "disconnected_work": {
          "title": "Desktop is disconnected from work",
          "description": "because open windows do not align with active work"
        },
        "missing_application": {
          "title": "Required application is missing",
          "description": "because Environment expects an application that is not present"
        }
      },
      "fallback": {
        "title": "Environment gap needs attention",
        "description": "because Environment reports a desktop–work gap"
      }
    }
  ],
  "signal_titles": {
    "outstanding_decision": "Outstanding decision needs attention",
    "blocked_action": "Blocked action needs attention",
    "blocked_task": "Blocked task needs attention",
    "waiting_task": "Waiting task needs attention",
    "in_progress_task": "In-progress task needs attention",
    "interrupted_work": "Interrupted work needs attention",
    "resumable_work": "Resumable work needs attention",
    "current_focus": "Current focus needs attention",
    "dormant_work": "Dormant work needs attention",
    "commitment_pending": "Pending commitment needs attention",
    "environment_disconnect": "Environment disconnect needs attention",
    "missing_application": "Missing application needs attention",
    "composition_gap": "Composition gap needs attention",
    "high_priority_intent": "High-priority intent needs attention",
    "purpose_obstacle": "Purpose obstacle needs attention",
    "purpose_outcome": "Purpose outcome needs attention",
    "evolution_insight": "Evolution insight needs attention",
    "activity_progress": "Recent activity needs attention",
    "recommendation_candidate": "Recommendation needs attention",
    "pattern_observation": "Pattern observation needs attention"
  },
  "contract_fixtures": [
    {
      "key": "task.base.blocked",
      "signal": "blocked_task",
      "source": "task_graph",
      "weight": 40,
      "known": true,
      "title": "Blocked task needs attention",
      "description": "because work is currently waiting on completion"
    },
    {
      "key": "decision.base.outstanding",
      "signal": "outstanding_decision",
      "source": "decision_queue",
      "weight": 55,
      "known": true,
      "title": "Outstanding decision needs attention",
      "description": "because a pending Decision Queue item still needs a human choice"
    },
    {
      "key": "purpose.obstacle.composition:missing_application",
      "signal": "purpose_obstacle",
      "source": "purpose",
      "weight": 36,
      "known": true,
      "title": "Purpose blocked by composition",
      "description": "because a Composition gap is obstructing Purpose progress"
    },
    {
      "key": "purpose.obstacle.unlisted_kind",
      "signal": "purpose_obstacle",
      "source": "purpose",
      "weight": 30,
      "known": true,
      "title": "Purpose obstacle needs attention",
      "description": "because Purpose reports an obstacle on the current path"
    },
    {
      "key": "task.base.unlisted_status",
      "signal": "waiting_task",
      "source": "task_graph",
      "weight": 20,
      "known": true,
      "title": "Open task needs attention",
      "description": "because Task Graph still lists this work as open"
    },
    {
      "key": "future.contract.unknown",
      "signal": "blocked_task",
      "source": "task_graph",
      "weight": 33,
      "known": false,
      "title": "Blocked task needs attention",
      "description": "No Experience translation for 'future.contract.unknown' yet (signal blocked_task from task_graph, weight 33)."
    }
  ]
} as const;
