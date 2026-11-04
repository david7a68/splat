---
splat_schema: 2
splat_id: e25bf248-a0a4-5827-99b4-6f30e12406ec
splat_kind: item
splat_key: dnd2024:items/ring_of_djinni_summoning
splat_name: Ring of Djinni Summoning
splat_origin: dnd2024:reference/magic_item/ring_of_djinni_summoning
splat_sources:
- SRD 5.2.1, pp. 237, https://www.dndbeyond.com/srd
---

# Ring of Djinni Summoning


## Item

| Field | Value |
| --- | --- |
| Type | Ring |
| Rarity | Legendary |
| Attunement | Yes |


## Source text

<!-- SRD page 237 -->

Ring, Legendary (Requires Attunement)

While wearing this ring, you can take a Magic action to summon a particular Djinni from the Elemental Plane of Air. The djinni appears in an unoccupied space you choose within 120 feet of yourself. It remains as long as you maintain Concentration, to a maximum of 1 hour, or until it drops to 0 Hit Points.

While summoned, the djinni is Friendly to you and your allies, and it obeys your commands. If you fail to command it, the djinni defends itself against attackers but takes no other actions.

After the djinni departs, it can’t be summoned again for 24 hours, and the ring becomes nonmagical if the djinni dies.

Rings of Djinni Summoning are often created by the djinn they summon and given to mortals as gifts of friendship or tokens of esteem.

Original layout: [[../../srd-5.2.1/pages/237|page 237]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/ring_of_djinni_summoning ` |
| ` implementation ` | ` dnd2024:item_ring_of_djinni_summoning ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` worn ` |
| ` action ` | ` magic ` |
| ` attunement_required ` | ` true ` |

#### Summon

<!-- splat:yaml path=splat_rule/parameters/summon -->

```yaml
creature:
  key: dnd2024:creatures/djinni
identity: same_particular_djinni
origin_plane: elemental_air
space:
  unoccupied: true
  choice: user
  range_ft: 120
duration:
  amount: 1
  unit: hours
concentration: true
ends_on:
- concentration_ends
- zero_hp
attitude: friendly_to_user_and_allies
obeys_commands: true
no_command_behavior: defend_self_otherwise_no_actions
```

#### Cooldown

<!-- splat:yaml path=splat_rule/parameters/cooldown -->

```yaml
from: djinni_departure
duration:
  amount: 24
  unit: hours
```

#### Djinni dies

<!-- splat:values path=splat_rule/parameters/djinni_dies -->

| Field | Value |
| --- | --- |
| ` item_magic ` | ` false ` |
