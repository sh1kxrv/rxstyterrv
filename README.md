# RxstyTerrv | Rusty Terra

> [!WARNING]
> 🚧 The application is in an active stage of development
> //

## How it should work?

### Examples

<table><tbody><tr><td width="500px"> Raw </td><td width="500px"> Transformed </td></tr><tr>
<td valign="top">

```js
// MDN
function getRandomIntInclusive(min, max) {
  const minCeiled = Math.ceil(min);
  const maxFloored = Math.floor(max);
  return Math.floor(Math.random() * (maxFloored - minCeiled + 1) + minCeiled);
}
```

</td><td valign="top">

```js
// WASM Stash
const a = window.a;
export function getRandomIntInclusive(min, max) {
  return a.b(min, max);
}
```

</td></tr></tbody></table>