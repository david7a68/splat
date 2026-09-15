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

<!-- splat:table
path: splat_rule/parameters/services
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Service:
    path: service
    type: symbol
  Requires any proficiency:
    path: requires_any_proficiency
    type: yaml
    optional: true
  Minimum gp per day:
    path: minimum_gp_per_day
    type: integer
    optional: true
  Requires proficiency:
    path: requires_proficiency
    type: boolean
    optional: true
  Minimum sp per day:
    path: minimum_sp_per_day
    type: integer
    optional: true
  Cp per mile:
    path: cp_per_mile
    type: integer
    optional: true
-->

| Key | Service | Requires any proficiency | Minimum gp per day | Requires proficiency | Minimum sp per day | Cp per mile |
| --- | --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` Skilled hireling ` | ` ["weapon", "tool", "skill"] ` | ` 2 ` |  |  |  |
| ` row_2 ` | ` Untrained hireling ` |  |  | false | ` 2 ` |  |
| ` row_3 ` | ` Messenger ` |  |  |  |  | ` 2 ` |

#### Expert price increase

<!-- splat:values path=splat_rule/parameters/expert_price_increase -->

| Field | Value |
| --- | --- |
| ` actor ` | ` gm ` |
| ` optional ` | ` true ` |
