# RxstyTerrv | Minimalistic JS VM

> [!WARNING]
> 🚧 The application is in an active stage of development
> //

## How it should work?

### Examples

<table><tbody><tr><td width="500px"> Raw </td><td width="500px"> Transformed </td></tr><tr>
<td valign="top">

```js
function add(a, b) {
  return a + b
}
```

</td><td valign="top">

```js
function add(a, b) {
  return $vm([0x0,0x0,0xA,0x6,0x1], a, b)
}
```

</td></tr></tbody></table>
