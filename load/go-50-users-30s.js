import http from 'k6/http';

export const options = {
  // A number specifying the number of VUs to run concurrently.
  vus: 50,
  // A string specifying the total duration of the test run.
  duration: '30s',
};

// The function that defines VU logic.
//
// See https://grafana.com/docs/k6/latest/examples/get-started-with-k6/ to learn more
// about authoring k6 scripts.
//
export default function() {
  http.get('http://localhost:8082/health');
}
