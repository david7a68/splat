# Inventory containers

> Historical decision record. The remaining schema details are now specified in
> [Mechanics](mechanics-schema.md), [State](state-schema.md), and
> [Campaign/storage](campaign-schema.md). Those contracts supersede the older
> illustrative fields and open-detail lists here. No implementation is authorized.

Status: container items, nesting, separate capacity and carried-weight rules,
and rule-defined access are agreed. Standalone setup notes and transfers that
preserve state are agreed. Named inventory tables with at most one unnamed root
table are agreed. Setup references, independent instances for each placement,
and one source for starting contents are agreed.
No data-model implementation is authorized.

Candidate state scoping, units and transfer details now appear in the
[schema draft](schema-draft.md), with a complete
[Supply Pack note](examples/library/Supply%20Pack.md). Those fields are a draft,
not additional implementation or registration of a package.

This builds on [Creatures and encounters](combat.md) and the agreed inventory
tables and per-item resources in [the creature example](creature-example.md).

## Required scope

An inventory is not limited to equipment carried by a creature. A container can
hold items whether carried or standing on its own. Its storage constraints
are separate from the carrying limit of a creature that might carry it.

Container support does not require maps, tokens, coordinates, or a spatial
engine. Those remain deferred. Owning a content definition in a package or
campaign is distinct from a creature physically holding a container in play.

## Container structure

Model containment as a capability of an item. A backpack and barrel can use
the same item model, with different rules. Each individual container has its
own inventory, and can be held by a creature, another container, or no creature.
Allow nested containers where rules permit. Disallow containment cycles, such
as putting a container inside itself through a chain of other containers.

Keep each physical item in one immediate containing inventory at a time. Moving
a whole container carries its contents with it; references describe the same
items rather than copied inventories. Transfers preserve state as described
below.

## Capacity and weight rules

Distinguish a container's capacity from the weight it contributes to its holder.
Capacity can constrain weight, volume, or item count, as specified by the item.
An ordinary container can contribute its own weight plus its contents, while a
magical container can define different rules for the weight presented outside.
The carrier's encumbrance uses that resulting weight.

Rules can also constrain what fits and whether particular nested containers
are allowed. Do not treat unknown weight or volume as zero when validating a
relevant limit. Exact dimensions, formulas, rule operations, and data requirements
remain open. These examples do not specify official item statistics.

## Access rules

Being present somewhere in a creature's inventory does not automatically make
an item immediately usable. Item and container rules determine requirements
for opening, retrieving, equipping, or using contents. These actions use the
same action timing and costs as other interactions.

This distinguishes a potion in a backpack from one that is ready to use,
without requiring physical distance calculations. Unknown access mechanics
remain subject to the existing unsupported-rule policy.

## Storage and transfers

### Named inventory tables

Use a separate marked table for each container so its contents remain together.
The following marker syntax is agreed:

```markdown
<!-- splat:inventory -->

| Key | Item | Quantity | Equipment |
| --- | --- | --- | --- |
| pack | `dnd2024:items/backpack` | 1 | worn |

<!-- splat:inventory container=pack -->

| Key | Item | Quantity | Equipment |
| --- | --- | --- | --- |
| potions | `dnd2024:items/potion_of_healing` | 3 | stored |
```

Allow at most one root table without a container parameter in a note. Every
additional table identifies its container by a stable inventory row key, not
the item's display name or the heading above the table. Keep row keys unique
across the note. Each container has at most one contents table, so an empty
container need not have an empty table.

A container row can occur in another container's table. For example, putting
`pouch` in the `pack` table and using `container=pouch` for its contents represents
nesting. Missing container rows, duplicate table targets, non-container targets,
and cycles are errors. The container's row placement determines its parent;
the contents marker determines which inventory a table describes.

Omit the Container column so there is one authority for
containment. It replaces the earlier allowance for several unqualified tables
forming one root inventory. Required item columns and supplementary state keyed
by row identity otherwise remain unchanged. Exact equipment-state vocabulary
is still open.

A container row must resolve to an item capable of containment. Containers
with their own contents have quantity one and separate item state. Supplementary
starting settings remain keyed by row in `splat_inventory_details` and do not
duplicate the containment relationships supplied by table markers. The resource
keys in the example are illustrative, not existing package entries.

### Standalone container notes

Allow a standalone container setup to be described in its own Markdown note,
referencing the reusable container item definition and using marked tables for
its contents. Its root inventory is the container's contents. It has no implicit
creature holder. A cellar barrel can be prepared without requiring a map.

Such notes are optional, not mandatory for every container. A creature can
carry containers defined directly by rows and contents tables in its own note.
An independently defined container can also be carried; storage in a separate
note does not mean it is permanently uncarried. Its carried row can reference
the setup as described below.

This is an authored starting setup. Each run creates its own container
and item instances from that setup. Saving changing world state remains outside
the already agreed initial persistence scope. References from encounters to
standalone containers remain open.

### Setup references and instances

Use
`splat_kind: container` and `splat_item` to reference its underlying item
definition. The reference is in the folded Splat data callout. Marked inventory
tables in the body define starting contents; this example has none:

````markdown
---
splat_schema: 2
splat_id: 96d4c5ca-6a2d-42a4-8cf7-9083f03f7bb2
splat_kind: container
splat_key: my_homebrew:containers/supply_pack
splat_name: Supply Pack
---


> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item -->
> 
> ```yaml
> key: dnd2024:items/backpack
> ```
````

Allow an inventory row's `Item` reference to target either an item
definition or a container setup. In the latter case the row receives a container
initialized from that setup, including its contents. The underlying definition
must support containment. Exact inherited starting-state fields remain open.

Each placement creates a separate runtime instance. Two guards
using Supply Pack each receive their own pack and contents. Transferring an
existing pack instead preserves that pack's identity and state, without creating
another instance from the setup. Reusing the setup note does not make the two
guards share one physical pack. Runtime instance IDs remain separate from note
UUIDs and local row keys.

Use one source for a container's starting contents: either a referenced
setup's tables or local tables attached to a row that references an item
definition. Do not silently merge inherited contents with an additional local
table for the same container. To customize a referenced setup, edit it or create
an independent campaign copy using the already agreed copy behavior.

Cycles between referenced setups must be detected as well as local containment
cycles. Exact validation and cross-note key scoping remain open.

### Transfers

Treat a transfer as one change that removes an item from its old inventory and
adds it to its new inventory. Preserve individual identity, all resource pools,
and nested contents. Carry out required access and capacity checks through the
applicable rules. A rejected transfer leaves the item's location and state
unchanged; action costs or failure consequences follow their own rules.

An explicitly chosen quantity can be split from a stack. Only compatible stacks
with matching relevant state can merge. Stateful individual items and containers
with contents retain their identities. Different inventories can use the same
local row key, so authored transfers need collision handling without confusing
runtime item identity with an editor's local row key.

Details of effect ownership, equipment changes, runtime identities, and
cross-note references remain open. Core transfer and explicit stack-splitting
behavior above is agreed.

## Design tree

- Containers and inventories independent of creature carrying capacity:
  required.
- Containers are items with their own inventories; nested containers are
  allowed by rules, with containment cycles prohibited: agreed.
- Container capacity is separate from the weight contributed to a holder;
  magical containers can alter the latter: agreed.
- Access to contents follows item/container rules and action costs: agreed.
- Optional standalone container setup notes are supported; creatures can still
  carry containers, and carried containers do not each need their own note:
  agreed.
- Transfers preserve item identity, pools, and contents; quantities can split
  explicitly and only compatible stacks can merge: agreed.
- Use one table per container, with `container=<row-key>` markers and at most
  one unnamed root table; omit the Container column: agreed.
- Setup notes use `splat_kind: container` and reference their underlying item
  through `splat_item`; inventory Item cells can reference a setup: agreed.
- Each placement creates independent runtime instances; transfers preserve
  the existing instance and its contents: agreed.
- Starting contents come from a referenced setup or local tables, without
  automatic merging; customize through editing or campaign copies: agreed.
- Next decisions: attack resolution in
  [the creature example](creature-example.md).
- Subsequent detail:
  - Runtime identity, inherited starting-state fields, and encounter placements.
  - Cross-note row-key scope and local row-key collisions on transfer.
  - Capacity units, unknown data, and rule-specific validation.
  - Interactions with existing equipment and resource-cost definitions.

Open details are not agreed behavior. No section authorizes implementation.
