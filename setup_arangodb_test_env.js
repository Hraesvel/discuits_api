const user = require('@arangodb/users');
const db_name = "discuits_test";
const user_name = 'discuits_test';

// Create test user
try {
    user.save(user_name, '');
} catch (error) {
    console.log(error);
}


// Create test database and documents
try {

    if (!db._databases().includes(db_name))
        db._createDatabase(db_name);
    user.grantDatabase(user_name, db_name, 'rw');
    db._useDatabase(db_name);
    var documents = {'album': 0, 'artist': 0, 'artist_to': 1, 'inventory': 0, 'variant': 0};
    Object.keys(documents).forEach(key => {
        try {
            switch (documents[key]) {
                case 0:
                    db._createDocumentCollection(key);
                    break;
                case 1:
                    db._createEdgeCollection(key);
                    break;
                default:
                    console.log("Woof");
            }
        } catch (e) {
            console.error("".concat("Error: Creating Collection ", key));
            console.error(e);
        }
    });


} catch (error) {
    console.log(error);
}
