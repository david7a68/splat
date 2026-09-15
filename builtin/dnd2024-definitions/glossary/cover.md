---
splat_schema: 1
splat_id: 48ec9594-2caf-545a-b733-4bafc310bca1
splat_kind: rule
splat_key: dnd2024:glossary/cover
splat_name: Cover
splat_origin:
  key: dnd2024:reference/glossary/cover
splat_sources:
- title: SRD 5.2.1
  locator: pp. 179
  url: https://www.dndbeyond.com/srd
---

# Cover

## Source text

<!-- SRD page 179 -->

Cover provides a degree of protection to a target\
behind it. There are three degrees of cover, each of\
which provides a different benefit to a target: Half\
Cover (+2 bonus to AC and Dexterity saving throws),\
Three-Quarters Cover (+5 bonus to AC and Dexter-\
ity saving throws), and Total Cover (can’t be tar-\
geted directly). If behind more than one degree of\
cover, a target benefits only from the most protec-\
tive degree. See also “Playing the Game” (“Combat”).\

Original layout: [[../../srd-5.2.1/pages/179|page 179]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/cover ` |
| ` implementation ` | ` dnd2024:glossary_cover ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` combining_degrees ` | ` most_protective_only ` |

#### Degrees

<!-- splat:yaml path=splat_rule/parameters/degrees -->

```yaml
half:
  ac_bonus: 2
  dexterity_save_bonus: 2
three_quarters:
  ac_bonus: 5
  dexterity_save_bonus: 5
total:
  direct_targeting_allowed: false
```
