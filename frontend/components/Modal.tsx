import { useState, useEffect } from "react";
import axios from "axios";

// Define the types for the order data
type OrderData = {
  menu_name: string;
  price: number;
  cook_time_seconds: number;
  ordered_time: string; // UTCであること
  check_staff_name: string;
};

// Define the types for the user object
type User = {
  token: string;
};

// Define the props for the Modal component
type ModalProps = {
  user: User;
  onClose: () => void;
  order_id: number;
};

const toMinutesAndSeconds = (totalSeconds: number) => {
  const minutes = Math.floor(totalSeconds / 60);
  const seconds = totalSeconds % 60;
  return `${minutes} minutes ${seconds} seconds`;
};

const toLocalTime = (utcTimeString: string) => {
  if (!utcTimeString) return "時間の値が不正です";

  // Create a Date object from the UTC time string
  const date = new Date(utcTimeString);

  if (isNaN(date.getTime())) return "無効な時間です";

  // Format the date to a string representing local time
  return date.toLocaleString("ja-JP", {
    timeZone: "Asia/Tokyo",
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
  });
};

const Modal: React.FC<ModalProps> = ({ user, onClose, order_id }) => {
  const [isLoading, setIsLoading] = useState(true);
  const [orderData, setOrderData] = useState<OrderData | null>(null);

  useEffect(() => {
    const getOrder = async (order_id: number) => {
      console.log("Order ID:", order_id);
      try {
        const response = await axios.get(`/api/order/${order_id}`, {
          headers: {
            Authorization: `Bearer ${user.token}`,
            "Content-Type": "application/json",
          },
        });
        console.log(response);
        setOrderData(response.data);
        setIsLoading(false);
      } catch (error) {
        console.error(`Error fetching data: ${error}`);
        alert(`Error fetching data: ${error}`);
        setIsLoading(false);
      }
    };

    getOrder(order_id);
  }, [order_id, user.token]);

  return (
    <div
      style={{
        position: "fixed",
        top: 0,
        left: 0,
        right: 0,
        bottom: 0,
        backgroundColor: "rgba(0,0,0,0.5)",
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
      }}
    >
      <div
        style={{
          backgroundColor: "white",
          padding: "20px",
          borderRadius: "8px",
        }}
      >
        {isLoading ? (
          <p>Loading...</p>
        ) : (
          <div>
            <div>{orderData?.menu_name}</div>
            <div>Price: {orderData?.price} yen</div>
            <div>
              Expected to take{" "}
              {orderData
                ? toMinutesAndSeconds(orderData.cook_time_seconds)
                : "N/A"}
            </div>
            <div>
              Ordered at:{" "}
              {orderData ? toLocalTime(orderData.ordered_time) : "N/A"}
            </div>
            <div>Checked by: {orderData?.check_staff_name}</div>
          </div>
        )}
        <br />
        <button onClick={onClose}>Close Modal</button>
      </div>
    </div>
  );
};

export default Modal;
