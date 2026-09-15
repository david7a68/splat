---
splat_schema: 1
splat_id: 80d03a57-0912-5ec7-b90b-e132a8876e05
splat_kind: rule
splat_key: dnd2024:glossary/opportunity_attacks
splat_name: Opportunity Attacks
splat_origin:
  key: dnd2024:reference/glossary/opportunity_attacks
splat_sources:
- title: SRD 5.2.1
  locator: pp. 185
  url: https://www.dndbeyond.com/srd
---

# Opportunity Attacks

## Source text

<!-- SRD page 185 -->

You can make an Opportunity Attack when a crea-\
ture that you can see leaves your reach using its\
action, its Bonus Action, its Reaction, or one of its\
speeds. To make the Opportunity Attack, take a Re-\
action to make one melee attack with a weapon or\
an Unarmed Strike against the provoking creature.\
The attack occurs right before the creature leaves\
your reach. See also “Playing the Game” (“Combat”).\

Original layout: [[../../srd-5.2.1/pages/185|page 185]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/opportunity_attacks ` |
| ` implementation ` | ` dnd2024:glossary_opportunity_attacks ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` cost ` | ` reaction ` |

#### Trigger

<!-- splat:values path=splat_rule/parameters/trigger -->

| Field | Value |
| --- | --- |
| ` creature_leaves_reach ` | ` true ` |
| ` subject_can_see_creature ` | ` true ` |
| ` movement_uses_any ` | ` ["action", "bonus_action", "reaction", "speed"] ` |

#### Attack

<!-- splat:values path=splat_rule/parameters/attack -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` type ` | ` melee ` |
| ` options ` | ` ["weapon", "unarmed_strike"] ` |
| ` target ` | ` provoking_creature ` |
| ` timing ` | ` immediately_before_leaving_reach ` |
