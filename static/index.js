document.addEventListener('DOMContentLoaded', function() {
    fetch('/get_data')
        .then(response => response.json())
        .then(data => {
            // document.getElementById('name').value = data[0].time;
            // document.getElementById('age').value = data[0].time;
        var tbody = document.querySelector("#jsonTable tbody");

        // Loop through JSON data and populate table rows
        data.forEach(function(item) {
            var row = document.createElement("tr");

            // Populate each cell in the row
            Object.values(item).forEach(function(value) {
                var cell = document.createElement("td");
                cell.textContent = value;
                row.appendChild(cell);
        });

        tbody.appendChild(row);
        })
        .catch(error => {
            console.error('Error fetching data:', error);
            alert('Error loading data. Please try again.');
        });
        
    });
});


