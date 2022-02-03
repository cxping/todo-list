## 基于 actix-web 本地josn的 todo-list 待办事件后端

### Api
#### Get:
http://localhost:8080/item/get
```
{
"done_item_count": 2,
"done_items": [
{
"title": "89000",
"status": "done"
},
{
"title": "washing",
"status": "done"
}
],
"pending_item_count": 2,
"pending_items": [
{
"title": "10090",
"status": "pending"
},
{
"title": "明天去哪里",
"status": "pending"
}
]
}
```
#### Create:
http://localhost:8080/item/create/{todo_titile}
```
http://localhost:8080/item/create/10090
```