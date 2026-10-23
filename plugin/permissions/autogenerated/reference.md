## Default Permission

What the page of a Krizaka Mac app may ask the native side: the folders the person granted (grant with the native picker,
revoke, list, watch), small secrets in the Keychain (the session token), the native menu, the deep links that arrived, the update channel.
No path is read without going through the folder scope; no file system, shell or opener permission is given to the page.

#### This default permission set includes the following:

- `allow-folders`
- `allow-grant-folder`
- `allow-revoke-folder`
- `allow-list-folder`
- `allow-watch-folder`
- `allow-unwatch-folder`
- `allow-secret-get`
- `allow-secret-set`
- `allow-secret-remove`
- `allow-set-menu`
- `allow-take-links`
- `allow-update-channel`
- `allow-set-update-channel`

## Permission Table

<table>
<tr>
<th>Identifier</th>
<th>Description</th>
</tr>


<tr>
<td>

`krizaka:allow-folders`

</td>
<td>

Enables the folders command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`krizaka:deny-folders`

</td>
<td>

Denies the folders command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`krizaka:allow-grant-folder`

</td>
<td>

Enables the grant_folder command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`krizaka:deny-grant-folder`

</td>
<td>

Denies the grant_folder command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`krizaka:allow-list-folder`

</td>
<td>

Enables the list_folder command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`krizaka:deny-list-folder`

</td>
<td>

Denies the list_folder command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`krizaka:allow-revoke-folder`

</td>
<td>

Enables the revoke_folder command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`krizaka:deny-revoke-folder`

</td>
<td>

Denies the revoke_folder command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`krizaka:allow-secret-get`

</td>
<td>

Enables the secret_get command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`krizaka:deny-secret-get`

</td>
<td>

Denies the secret_get command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`krizaka:allow-secret-remove`

</td>
<td>

Enables the secret_remove command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`krizaka:deny-secret-remove`

</td>
<td>

Denies the secret_remove command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`krizaka:allow-secret-set`

</td>
<td>

Enables the secret_set command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`krizaka:deny-secret-set`

</td>
<td>

Denies the secret_set command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`krizaka:allow-set-menu`

</td>
<td>

Enables the set_menu command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`krizaka:deny-set-menu`

</td>
<td>

Denies the set_menu command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`krizaka:allow-set-update-channel`

</td>
<td>

Enables the set_update_channel command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`krizaka:deny-set-update-channel`

</td>
<td>

Denies the set_update_channel command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`krizaka:allow-take-links`

</td>
<td>

Enables the take_links command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`krizaka:deny-take-links`

</td>
<td>

Denies the take_links command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`krizaka:allow-unwatch-folder`

</td>
<td>

Enables the unwatch_folder command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`krizaka:deny-unwatch-folder`

</td>
<td>

Denies the unwatch_folder command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`krizaka:allow-update-channel`

</td>
<td>

Enables the update_channel command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`krizaka:deny-update-channel`

</td>
<td>

Denies the update_channel command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`krizaka:allow-watch-folder`

</td>
<td>

Enables the watch_folder command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`krizaka:deny-watch-folder`

</td>
<td>

Denies the watch_folder command without any pre-configured scope.

</td>
</tr>
</table>
