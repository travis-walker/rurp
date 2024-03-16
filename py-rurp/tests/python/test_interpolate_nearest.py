import rurp


def test_it_works():
    data = rurp.interpolate.apply_nearest_neighbor_interpolation()
    assert data is not None
