"use client";

import Head from "next/head";
import axios from "axios";
import styles from "@/styles/Order.module.css";
import { Inter } from "next/font/google";
import { useEffect, useState } from "react";
import { useRouter } from "next/router";
import { useAuth } from "../context/AuthContext";
import Countdown from "../components/Countdown";
import Modal from "../components/Modal";

const inter = Inter({ subsets: ["latin"] });

type Order = {
  order_id: number;
  menu_name: string;
  expected_cook_finish_time: string;
};

type User = {
  token: string;
};

export default function Order() {
  const router = useRouter();
  const { user, loading } = useAuth();
  const [orders, setOrders] = useState<Order[]>([]);
  const [order_id, setOrder] = useState<number | null>(null);
  const [tableId, setTableId] = useState<number | null>(null);
  const [isSending, setIsSending] = useState<boolean>(false);
  const [isModalOpen, setModalOpen] = useState<boolean>(false);

  useEffect(() => {
    const tableIdQuery = router.query.tableId as string | undefined;

    // Loading状態の間は処理をしない
    if (loading) return;

    if (!loading && !user) {
      router.push("/login");
    }
    if (!loading && user && tableIdQuery) {
      const numericTableId = parseInt(tableIdQuery, 10);
      setTableId(numericTableId);
      axios
        .get<Order[]>(`/api/table/${numericTableId}/order`, {
          headers: {
            Authorization: `Bearer ${user.token}`,
          },
        })
        .then((res) => {
          console.log(res.data);
          setOrders(res.data);
        })
        .catch((error) => {
          alert(`Error fetching data: ${error}`);
        });
    }
  }, [user, loading, router]);

  const handleOpenModal = (order_id: number) => {
    setOrder(order_id);
    setModalOpen(true);
  };

  const handleCloseModal = () => {
    setModalOpen(false);
  };

  const deleteOrder = (order_id: number) => {
    setIsSending(true);
    console.log("Order ID:", order_id);
    axios
      .delete(`/api/order`, {
        headers: {
          Authorization: `Bearer ${user?.token}`,
          "Content-Type": "application/json",
        },
        data: {
          order_id: order_id,
        },
      })
      .then((res) => {
        console.log(res.data);
        setOrders((prevOrders) =>
          prevOrders.filter((order) => order.order_id !== order_id)
        );
        setIsSending(false);
        alert(`Successfully canceled`);
      })
      .catch((error) => {
        setIsSending(false);
        alert(`Error fetching data: ${error}`);
      });
  };

  const serveOrder = (order: Order, order_id: number, menu_name: string) => {
    setIsSending(true);
    console.log("Order: ", order);
    console.log("Order ID:", order_id);
    axios
      .delete(`/api/order/complete`, {
        headers: {
          Authorization: `Bearer ${user?.token}`,
          "Content-Type": "application/json",
        },
        data: {
          order_id: order_id,
        },
      })
      .then((res) => {
        console.log(res.data);
        setOrders((prevOrders) =>
          prevOrders.filter((order) => order.order_id !== order_id)
        );
        setIsSending(false);
        alert(`Successfully served "${menu_name}"`);
      })
      .catch((error) => {
        setIsSending(false);
        alert(`Error fetching data: ${error}`);
      });
  };

  const deleteAllOrders = (tableId: number) => {
    setIsSending(true);
    console.log("Table ID:", tableId);
    axios
      .delete(`/api/table/order`, {
        headers: {
          Authorization: `Bearer ${user?.token}`,
          "Content-Type": "application/json",
        },
        data: {
          restaurant_table_id: tableId,
        },
      })
      .then((res) => {
        console.log(res.data);
        setOrders([]);
        setIsSending(false);
        alert(`Successfully canceled all orders`);
      })
      .catch((error) => {
        setIsSending(false);
        alert(`Error fetching data: ${error}`);
      });
  };

  const goToMenuPage = (tableId: number) => {
    console.log("Table ID:", tableId);
    router.push(`/menu?tableId=${tableId}`);
  };

  const toLocalTime = (utcTimeString: string | undefined) => {
    if (!utcTimeString) {
      return "不正な値です";
    }

    const date = new Date(ensureUtc(utcTimeString));
    return date.toLocaleString();
  };

  const ensureUtc = (timeString: string) => {
    // `timeString` が undefined または null でないことを確認
    if (!timeString) {
      return timeString + "Z";
    }
    return timeString || ""; // `timeString` が `undefined` の場合は空文字を返す
  };

  return (
    <>
      <Head>
        <title>Restaurant App</title>
        <meta name="description" content="Restaurant App" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main className={`${styles.main} ${inter.className}`}>
        <h2>Table {tableId}</h2>
        {isModalOpen && user && (
          <Modal
            user={user}
            onClose={handleCloseModal}
            order_id={order_id ?? 0}
          />
        )}
        <ul>
          {orders.length === 0 ? (
            <div>No order found</div>
          ) : (
            orders.map((order) => (
              <li className={`${styles.order}`} key={order.order_id}>
                <ul>
                  <li>{order.menu_name}</li>
                  <li>
                    Expected to finish cooking at:{" "}
                    {toLocalTime(order.expected_cook_finish_time)}
                  </li>
                  <li>
                    <Countdown
                      targetTimeString={toLocalTime(
                        order.expected_cook_finish_time
                      )}
                    />
                  </li>
                  <li>
                    <button
                      className={`${styles.deleteOrderButton}`}
                      type="button"
                      onClick={() => deleteOrder(order.order_id)}
                      disabled={isSending}
                    >
                      Cancel order
                    </button>
                    <button
                      className={`${styles.deleteOrderButton}`}
                      type="button"
                      disabled={isSending}
                      onClick={() =>
                        serveOrder(order, order.order_id, order.menu_name)
                      }
                    >
                      Serve
                    </button>
                    <button
                      className={`${styles.deleteOrderButton}`}
                      type="button"
                      disabled={isSending}
                      onClick={() => handleOpenModal(order.order_id)}
                    >
                      Detail
                    </button>
                  </li>
                </ul>
              </li>
            ))
          )}
        </ul>
        <div>
          <button type="button" onClick={() => goToMenuPage(tableId ?? 0)}>
            Menu
          </button>
          <button
            type="button"
            className={`${styles.deleteOrderButton}`}
            onClick={() => deleteAllOrders(tableId ?? 0)}
          >
            Delete all orders
          </button>
        </div>
      </main>
    </>
  );
}
