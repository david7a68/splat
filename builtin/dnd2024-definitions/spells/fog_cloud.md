---
splat_schema: 2
splat_id: 9dfe0dcf-e87a-58f6-9ba7-b10786b48c38
splat_kind: spell
splat_key: dnd2024:spells/fog_cloud
splat_name: Fog Cloud
splat_origin: dnd2024:reference/spell/fog_cloud
splat_sources:
- SRD 5.2.1, pp. 133, https://www.dndbeyond.com/srd
---

# Fog Cloud


## Spell

| Field | Value |
| --- | --- |
| Level | 1 |
| School | Conjuration |
| Classes | Druid, Ranger, Sorcerer, Wizard |
| Casting Time | Action |
| Range | 120 feet |
| Components | V, S |
| Duration | Concentration, up to 1 hour |


## Source text

<!-- SRD page 133 -->

Level 1 Conjuration (Druid, Ranger, Sorcerer, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S\
Duration: Concentration, up to 1 hour

You create a 20-foot-radius Sphere of fog centered on a point within range. The Sphere is Heavily Obscured. It lasts for the duration or until a strong wind (such as one created by Gust of Wind) disperses it.

Using a Higher-Level Spell Slot. The fog’s radius increases by 20 feet for each spell slot level above 1.

Original layout: [[../../srd-5.2.1/pages/133|page 133]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/fog_cloud ` |
| ` implementation ` | ` dnd2024:spell_fog_cloud ` |

### Parameters

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` sphere ` |
| ` radius_feet ` | ` 20 ` |
| ` heavily_obscured ` | ` true ` |

#### End

<!-- splat:values path=splat_rule/parameters/end -->

| Field | Value |
| --- | --- |
| ` strong_wind ` | ` disperse ` |

#### Scaling

<!-- splat:values path=splat_rule/parameters/scaling -->

| Field | Value |
| --- | --- |
| ` per_slot_above ` | ` 1 ` |
| ` radius_feet ` | ` 20 ` |
