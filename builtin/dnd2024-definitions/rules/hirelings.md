---
splat_schema: 1
splat_id: 168e6643-1082-540d-b373-7485412627b1
splat_kind: rule
splat_key: dnd2024:rules/hirelings
splat_name: Hirelings
splat_origin:
  key: dnd2024:reference/pages/102
splat_sources:
- title: SRD 5.2.1
  locator: pp. 102
  url: https://www.dndbeyond.com/srd
---

# Hirelings

## Source text

<!-- SRD page 102 -->

Skilled hirelings include anyone hired to perform\
a service that involves a proficiency (including\
weapon, tool, or skill): a mercenary, an artisan, a\
scribe, or the like. The pay shown on the Hirelings\
table is a minimum; some expert hirelings require\
more pay. Untrained hirelings are hired for work\
that requires no particular proficiencies; they in-\
clude laborers and porters.\
Hirelings\
Service Cost\
Skilled hireling 2 GP per day\
Untrained hireling 2 SP per day\
Messenger 2 CP per mile\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/hirelings ` |
| ` implementation ` | ` dnd2024:rule_hirelings ` |

### Parameters

#### Services

<!-- splat:rows path=splat_rule/parameters/services -->

| Row | service | requires_any_proficiency | minimum_gp_per_day | requires_proficiency | minimum_sp_per_day | cp_per_mile |
| --- | --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` skilled_hireling ` | ` ["weapon", "tool", "skill"] ` | ` 2 ` |  |  |  |
| ` 1 ` | ` untrained_hireling ` |  |  | ` false ` | ` 2 ` |  |
| ` 2 ` | ` messenger ` |  |  |  |  | ` 2 ` |

#### Expert price increase

<!-- splat:values path=splat_rule/parameters/expert_price_increase -->

| Field | Value |
| --- | --- |
| ` actor ` | ` gm ` |
| ` optional ` | ` true ` |
