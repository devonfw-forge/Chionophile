fn get<T: FromStr>(path: &str, position: usize) -> Result<T, T::Err> {
  let split_vec = path.split('/').collect();

  return split_vec[position].parse();
}