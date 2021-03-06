use smartcore::dataset::iris;
use smartcore::linalg::naive::dense_matrix::DenseMatrix;
use smartcore::linear::logistic_regression::LogisticRegression;
use smartcore::model_selection::train_test_split;
use smartcore::metrics::mean_absolute_error;
use smartcore::neighbors::knn_classifier::KNNClassifier;

fn main() {
    let iris_data = iris::load_dataset();
    let x = DenseMatrix::from_array(
        iris_data.num_samples,
        iris_data.num_features,
        &iris_data.data,
        );

    let y = iris_data.target;

    let (x_train, x_test, y_train, y_test) = train_test_split(&x, &y, 0.2, true);

    // Logisitic regression
    let logisitic_reg = LogisticRegression::fit(
        &x_train,
        &y_train,
        Default::default(),
        ).unwrap();

    let result_logisitc_reg = logisitic_reg.predict(&x_test).unwrap();

    assert!(mean_absolute_error(&y_test, &result_logisitc_reg) < 0.2);

    // KNN 
    let knn = KNNClassifier::fit(
        &x_train,
        &y_train,
        Default::default(),
        ).unwrap();

    let result = knn.predict(&x_test).unwrap();

    assert!(mean_absolute_error(&y_test, &result) < 0.2);
}
