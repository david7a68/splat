---
splat_schema: 2
splat_id: b3344347-6f56-5da9-962b-ee4c8eb3c8c6
splat_kind: spell
splat_key: dnd2024:spells/transport_via_plants
splat_name: Transport via Plants
splat_origin: dnd2024:reference/spell/transport_via_plants
splat_sources:
- SRD 5.2.1, pp. 170, https://www.dndbeyond.com/srd
---

# Transport via Plants


## Spell

| Field | Value |
| --- | --- |
| Level | 6 |
| School | Conjuration |
| Classes | Druid |
| Casting Time | Action |
| Range | 10 feet |
| Components | V, S |
| Duration | 1 minute |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 170 -->

Level 6 Conjuration (Druid)\
Casting Time: Action\
Range: 10 feet\
Components: V, S\
Duration: 1 minute

This spell creates a magical link between a Large or larger inanimate plant within range and another plant, at any distance, on the same plane of existence. You must have seen or touched the destination plant at least once before. For the duration, any creature can step into the target plant and exit from the destination plant by using 5 feet of movement.

Original layout: [[../../srd-5.2.1/pages/170|page 170]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/transport_via_plants ` |
| ` implementation ` | ` dnd2024:spell_transport_via_plants ` |

### Parameters

#### Targeting

<!-- splat:yaml path=splat_rule/parameters/targeting -->

```yaml
origin:
  kind: inanimate_plant
  size_min: large
  within_range: true
destination:
  kind: plant
  same_plane: true
  distance_limit: none
  previously_seen_or_touched: true
```

#### Link

<!-- splat:values path=splat_rule/parameters/link -->

| Field | Value |
| --- | --- |
| ` duration ` | ` spell_duration ` |
| ` eligible_traveler ` | ` any_creature ` |
| ` direction ` | ` origin_to_destination ` |
| ` movement_cost_feet ` | ` 5 ` |
| ` enter ` | ` origin_plant ` |
| ` exit ` | ` destination_plant ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  value: 1
  unit: minute
```
