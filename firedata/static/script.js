window.onload = loadTable;

async function loadTable() {
  const response = await fetch('/get_table');
  if (response.ok) {
    const tableHtml = await response.text();
    document.getElementById('item-table').innerHTML = tableHtml;
  } else {
    alert('Failed to load table');
  }
}

async function sendForm() {
    let form = document.getElementById('item-form');
    let formData = new FormData(form);
    let jsonData = {};
    formData.forEach((value, key) => {
        jsonData[key] = value;
    });

    let response = await fetch('/add_item', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(jsonData)
    });

    if (response.ok) {
        alert('Item added successfully');
        // Fetch and update the table without reloading
        loadTable();
    } else {
        let errorText = await response.text();
        alert('Error adding item: ' + errorText);
    }
}
